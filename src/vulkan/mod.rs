//! The voxelar vulkan backend.
//!
//! This is the module that provides Vulkan-related abstractions and functionality.
//!
//! Module overview:
//! - buffers: Provides all abstractions for Vulkan buffers
//! - command: Provides all abstractions and functionality for Vulkan commands
//! - creation\_info: Provides a `DataStructureCreationInfo` struct for high-level information related to the `VulkanContext` data structure initialization
//! - debug: Provides an abstraction for the verification layer setup (if requested)
//! - depth\_image: Provides an abstraction for depth image creation
//! - descriptors: Provides all descriptor logic and abstractions
//! - dynamic\_uniform\_buffer: Provides an abstraction for uniform buffers that can be used with dynamic descriptor sets
//! - egui\_integration: A wrapper for the egui integration provided by the `egui-winit-ash-integration` crate
//! - frame\_data: Provides an abstraction for per-frame synchronization and command logic in double/triple/...-buffering scenarios
//! - framebuffers: Provides an abstraction for framebuffer creation for each present image of a swapchain
//! - graphics\_pipeline\_builder: Provides an abstraction for building Vulkan `Pipeline`s
//! - image: Provides abstractions for all image-related functionality
//! - logical\_device: Provides a wrapper around logical Vulkan devices
//! - per\_frame: Provides an abstraction for tracking data of each frame in double/triple/...-buffering scenarios; used with `FrameData` in this module
//! - physical\_device: Provides an abstraction for finding a suitable `PhysicalDevice` for rendering, also queries important device information
//! - pipeline\_layout: Provide a wrapper around `PipelineLayout`s
//! - pipeline\_layout\_builder: Provides an abstraction for building `(SetUp)PipelineLayout`s
//! - present\_images: Provides an abstraction for getting the images of a swapchain
//! - render\_pass: Provides an abstraction for the creation of a default render pass
//! - shader: Provides an abstraction for shader compilation and shader module creation
//! - surface: Provides an abstraction for the window surface and all related information
//! - swapchain: Provides an abstraction for the creation of a default swapchain
//! - sync: Provides a wrapper around synchronization structures (related to rendering)
//! - util: Provides random utility functions used by the vulkan module

use std::ffi::{c_char, CStr, CString};
use std::mem::ManuallyDrop;
use std::sync::{Arc, Mutex, MutexGuard};

use ash::vk::ApplicationInfo;
use ash::vk::ClearValue;
use ash::vk::Filter;
use ash::vk::Format;
use ash::vk::PipelineStageFlags;
use ash::vk::PresentInfoKHR;
use ash::vk::SamplerAddressMode;
use ash::vk::ShaderStageFlags;
use ash::vk::{CommandBufferLevel, CommandPoolResetFlags};
use ash::vk::{Extent2D, Extent3D};
use ash::vk::{Fence, FenceCreateFlags};
use ash::vk::{InstanceCreateFlags, InstanceCreateInfo};
use ash::vk::{RenderPassBeginInfo, SubpassContents};
use ash::{Entry, Instance};
use gpu_allocator::vulkan::*;
use gpu_allocator::*;

use paste::paste;

#[cfg(any(target_os = "macos", target_os = "ios"))]
use ash::vk::{KhrGetPhysicalDeviceProperties2Fn, KhrPortabilityEnumerationFn};

pub mod buffers;
pub mod command;
pub mod creation_info;
pub mod debug;
pub mod depth_image;
pub mod descriptors;
pub mod egui_integration;
pub mod frame_data;
pub mod framebuffers;
pub mod graphics_pipeline_builder;
pub mod image;
pub mod logical_device;
pub mod per_frame;
pub mod physical_device;
pub mod pipeline_layout;
pub mod pipeline_layout_builder;
pub mod present_images;
pub mod render_pass;
pub mod shader;
pub mod surface;
pub mod swapchain;
pub mod sync;
pub mod util;

use crate::render_context::RenderContext;
use crate::result::Context;
use crate::window::{VoxelarEventLoop, VoxelarWindow};
use crate::Voxelar;

use crate::vulkan::per_frame::PerFrame;

use self::buffers::staging_buffer::SetUpStagingBuffer;
use self::buffers::storage_buffer::SetUpStorageBuffer;
use self::buffers::typed_buffer::TypedAllocatedBuffer;
use self::buffers::uniform_buffer::SetUpUniformBuffer;
use self::command::command_buffer::SetUpCommandBufferWithFence;
use self::command::command_pool::SetUpCommandPool;
use self::creation_info::DataStructureCreationInfo;
use self::creation_info::PresentModeInitMode;
use self::debug::VerificationProvider;
use self::depth_image::SetUpDepthImage;
use self::egui_integration::SetUpEguiIntegration;
use self::frame_data::FrameData;
use self::framebuffers::SetUpFramebuffers;
use self::image::sampler::SetUpSampler;
use self::image::texture::Texture;
use self::logical_device::SetUpLogicalDevice;
use self::physical_device::SetUpPhysicalDevice;
use self::present_images::SetUpPresentImages;
use self::render_pass::SetUpRenderPass;
use self::shader::CompiledShaderModule;
use self::surface::SetUpSurfaceInfo;
use self::swapchain::SetUpSwapchain;

pub struct VulkanContext {
    pub entry: Entry,
    pub instance: Instance,
    pub surface_info: SetUpSurfaceInfo,

    pub verification: Box<dyn VerificationProvider>,

    pub last_creation_info: Option<DataStructureCreationInfo>,
    pub physical_device: Option<SetUpPhysicalDevice>,
    pub logical_device: Option<SetUpLogicalDevice>,
    pub allocator: Option<ManuallyDrop<Arc<Mutex<Allocator>>>>, // This type is interesting
    pub swapchain: Option<SetUpSwapchain>,
    pub present_images: Option<SetUpPresentImages>,
    pub command_pool_for_setup: Option<SetUpCommandPool>,
    pub depth_image: Option<SetUpDepthImage>,
    pub render_pass: Option<SetUpRenderPass>,
    pub framebuffers: Option<SetUpFramebuffers>,

    pub frames: PerFrame<FrameData>,
}

macro_rules! generate_safe_getter {
    ($name:ident, $type:ty, $err_message:tt) => {
        pub fn $name(&self) -> crate::Result<&$type> {
            self.$name.as_ref().context($err_message.to_string())
        }

        paste! {
            pub fn [<$name _mut>](&mut self) -> crate::Result<&mut $type> {
                self.$name.as_mut().context($err_message.to_string())
            }
        }
    };
}

impl VulkanContext {
    fn create_app_info(window: &VoxelarWindow) -> ApplicationInfo {
        let app_name = CString::new(window.get_title()).unwrap();

        let app_info = ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(0)
            .engine_name(&app_name)
            .engine_version(0)
            .api_version(ash::vk::make_api_version(0, 1, 0, 0));

        *app_info
    }

    generate_safe_getter!(
        physical_device,
        SetUpPhysicalDevice,
        "No physical device was set up yet! Use VulkanContext::find_usable_physical_device to do so"
    );

    generate_safe_getter!(
        logical_device,
        SetUpLogicalDevice,
        "No logical device was set up yet! Use VulkanContext::create_logical_device to do so"
    );

    pub fn lock_allocator(&self) -> crate::Result<MutexGuard<Allocator>> {
        let allocator = self.allocator.as_ref().context(
            "No allocator was set up yet! Use VulkanContext::create_allocator to do so".to_string(),
        )?;
        Ok(allocator
            .lock()
            .context("Unable to acquire allocator mutex lock".to_string())?)
    }

    pub fn create_allocator_ref(&self) -> crate::Result<Arc<Mutex<Allocator>>> {
        let allocator = self.allocator.as_ref().context(
            "No allocator was set up yet! Use VulkanContext::create_allocator to do so".to_string(),
        )?;
        Ok(Arc::clone(&allocator))
    }

    generate_safe_getter!(
        swapchain,
        SetUpSwapchain,
        "No swapchain was set up yet! Use VulkanContext::create_swapchain to do so"
    );

    generate_safe_getter!(
        present_images,
        SetUpPresentImages,
        "No present image were set up yet! Use VulkanContext::create_present_images to do so"
    );

    generate_safe_getter!(
        command_pool_for_setup,
        SetUpCommandPool,
        "No command logic for setup commands was set up yet! Use VulkanContext::create_command_pool_for_setup to do so"
    );

    generate_safe_getter!(
        depth_image,
        SetUpDepthImage,
        "No depth image was set up yet! Use VulkanContext::create_depth_image to do so"
    );

    generate_safe_getter!(
        render_pass,
        SetUpRenderPass,
        "No render pass was set up yet! Use VulkanContext::create_render_pass to do so"
    );

    generate_safe_getter!(
        framebuffers,
        SetUpFramebuffers,
        "No framebuffers were set up yet! Use VulkanContext::create_framebuffers to do so"
    );

    pub fn find_usable_physical_device(&mut self) -> crate::Result<()> {
        unsafe {
            self.physical_device = Some(SetUpPhysicalDevice::find_usable_device(
                &self.instance,
                &self.surface_info,
            )?);
        }
        Ok(())
    }

    pub fn create_logical_device(&mut self) -> crate::Result<()> {
        unsafe {
            self.logical_device = Some(SetUpLogicalDevice::create_with_defaults(
                &self.instance,
                self.physical_device()?,
            )?);
        }

        Ok(())
    }

    pub fn create_allocator(
        &mut self,
        debug_settings: AllocatorDebugSettings,
    ) -> crate::Result<()> {
        let allocator = Allocator::new(&AllocatorCreateDesc {
            instance: self.instance.clone(),
            device: self.logical_device()?.device.clone(),
            physical_device: self.physical_device()?.physical_device,
            debug_settings,
            buffer_device_address: false,
        })?;
        let allocator = Arc::new(Mutex::new(allocator));
        self.allocator = Some(ManuallyDrop::new(allocator));
        Ok(())
    }

    pub fn create_new_swapchain(
        &mut self,
        present_mode_init_mode: PresentModeInitMode,
    ) -> crate::Result<SetUpSwapchain> {
        unsafe {
            let new_swapchain = SetUpSwapchain::create_with_defaults(
                &self.instance,
                &self.surface_info,
                self.logical_device()?,
                present_mode_init_mode,
                self.swapchain.as_ref(),
            )?;

            Ok(new_swapchain)
        }
    }

    pub fn create_swapchain(
        &mut self,
        present_mode_init_mode: PresentModeInitMode,
    ) -> crate::Result<()> {
        self.swapchain = Some(self.create_new_swapchain(present_mode_init_mode)?);
        Ok(())
    }

    pub fn create_present_images(&mut self) -> crate::Result<()> {
        unsafe {
            self.present_images = Some(SetUpPresentImages::create_with_defaults(
                self.logical_device()?,
                self.swapchain()?,
                &self.surface_info,
            )?);
        }

        Ok(())
    }

    pub fn create_command_pool_for_setup(&mut self) -> crate::Result<()> {
        unsafe {
            self.command_pool_for_setup = Some(SetUpCommandPool::create(
                self.logical_device()?,
                1,
                CommandBufferLevel::PRIMARY,
                FenceCreateFlags::empty(),
            )?);
            Ok(())
        }
    }

    pub fn create_depth_image(&mut self) -> crate::Result<()> {
        unsafe {
            let mut depth_image = {
                let allocator = &mut self.lock_allocator()?;
                SetUpDepthImage::create_with_defaults(
                    self.logical_device()?,
                    allocator,
                    &self.surface_info,
                )?
            };

            self.submit_immediate_setup_commands(|device, setup_command_buffer| {
                depth_image
                    .perform_layout_transition_pipeline_barrier(device, setup_command_buffer);
                Ok(())
            })?;

            self.depth_image = Some(depth_image);
        }
        Ok(())
    }

    pub fn create_framebuffers(&mut self) -> crate::Result<()> {
        unsafe {
            self.framebuffers = Some(SetUpFramebuffers::create(
                self.logical_device()?,
                self.depth_image()?,
                &self.surface_info,
                self.present_images()?,
                self.render_pass()?,
            )?);
        }

        Ok(())
    }

    pub fn create_render_pass(&mut self) -> crate::Result<()> {
        unsafe {
            self.render_pass = Some(SetUpRenderPass::create_with_defaults(
                self.logical_device()?,
                &self.surface_info,
            )?);
        }

        Ok(())
    }

    pub fn create_per_frame_data(&mut self, frame_overlap: u32) -> crate::Result<()> {
        unsafe {
            let logical_device = self.logical_device()?;
            self.frames = PerFrame::try_init(
                |_| FrameData::create_with_defaults(logical_device),
                frame_overlap as usize,
            )?;
            Ok(())
        }
    }

    pub fn create_default_data_structures(
        &mut self,
        window_size: (u32, u32),
        creation_info: DataStructureCreationInfo,
    ) -> crate::Result<()> {
        self.find_usable_physical_device()?;
        if let Some(physical_device) = &self.physical_device {
            self.surface_info.update(physical_device, window_size)?;
        }
        self.create_logical_device()?;
        self.create_allocator(creation_info.allocator_debug_settings)?;
        self.create_swapchain(creation_info.swapchain_present_mode)?;
        self.create_present_images()?;
        self.create_command_pool_for_setup()?;
        self.create_depth_image()?;
        self.create_render_pass()?;
        self.create_framebuffers()?;
        self.create_per_frame_data(creation_info.frame_overlap)?;
        self.last_creation_info = Some(creation_info);

        Ok(())
    }

    pub fn update_swapchain(&mut self, window_size: (u32, u32)) -> crate::Result<()> {
        let creation_info = self.last_creation_info.context(
            "No last creation info was set, so data structures can't be recreated".to_string(),
        )?;

        if let Some(physical_device) = &self.physical_device {
            self.surface_info.update(physical_device, window_size)?;
        }

        let new_swapchain = self.create_new_swapchain(creation_info.swapchain_present_mode)?;

        if let Some(logical_device) = self.logical_device.as_ref() {
            logical_device.wait()?;

            if let Some(render_pass) = self.render_pass.as_mut() {
                render_pass.destroy(&logical_device);
            }

            if let Some(framebuffers) = self.framebuffers.as_mut() {
                framebuffers.destroy(&logical_device);
            }

            if let Some(mut depth_image) = self.depth_image.take() {
                let mut allocator = self.lock_allocator()?;
                depth_image.destroy(&logical_device, &mut allocator)?;
            }

            if let Some(command_pool_for_setup) = self.command_pool_for_setup.as_mut() {
                command_pool_for_setup.destroy(&logical_device);
            }

            if let Some(present_images) = self.present_images.as_mut() {
                present_images.destroy(&logical_device);
            }

            for frame in self.frames.iter_mut() {
                frame.destroy(&logical_device);
            }

            if let Some(swapchain) = self.swapchain.as_mut() {
                swapchain.destroy();
            }
        }

        self.swapchain = Some(new_swapchain);
        self.create_present_images()?;
        self.create_command_pool_for_setup()?;
        self.create_depth_image()?;
        self.create_render_pass()?;
        self.create_framebuffers()?;
        self.create_per_frame_data(creation_info.frame_overlap)?;

        Ok(())
    }

    pub fn create_egui_integration(
        &self,
        window: &VoxelarWindow,
        event_loop: &VoxelarEventLoop,
    ) -> crate::Result<SetUpEguiIntegration> {
        let size = window.get_size();
        let logical_device = self.logical_device()?;
        let allocator = self.create_allocator_ref()?;
        let surface_format = self.surface_info.surface_format(0)?;
        Ok(SetUpEguiIntegration::new(
            &event_loop.event_loop,
            size.0 as u32,
            size.1 as u32,
            window.scale_factor(),
            logical_device,
            allocator,
            self.physical_device()?.queue_family_index,
            logical_device.present_queue,
            self.swapchain()?,
            surface_format,
        ))
    }

    pub fn update_egui_integration_swapchain(
        &self,
        window_size: (u32, u32),
        egui_integration: &mut SetUpEguiIntegration,
    ) -> crate::Result<()> {
        egui_integration.update_swapchain(
            window_size.0,
            window_size.1,
            self.swapchain()?,
            &self.surface_info,
        )
    }
}

impl VulkanContext {
    pub fn get_surface_extent(&self) -> crate::Result<Extent2D> {
        self.surface_info.surface_extent()
    }

    pub fn frame_overlap(&self) -> usize {
        self.frames.len()
    }

    pub fn submit_immediate_setup_commands<F>(&self, command_buffer_op: F) -> crate::Result<()>
    where
        F: FnOnce(&SetUpLogicalDevice, &SetUpCommandBufferWithFence) -> crate::Result<()>,
    {
        let command_pool = self.command_pool_for_setup()?;

        let setup_command_buffer = command_pool.get_command_buffer(0);
        let logical_device = self.logical_device()?;
        let present_queue = logical_device.present_queue;
        setup_command_buffer
            .record_commands_for_one_time_submit(logical_device, command_buffer_op)?;
        setup_command_buffer.submit(logical_device, present_queue, &[], &[], &[])?;
        setup_command_buffer.wait_for_fence(logical_device)?;
        setup_command_buffer.reset_fence(logical_device)?;

        command_pool.reset(logical_device, CommandPoolResetFlags::empty())
    }

    pub fn select_frame(&mut self, current_frame_index: usize) {
        self.frames.select(current_frame_index);
    }

    pub fn wait_for_current_frame_draw_buffer_fences(&self) -> crate::Result<()> {
        let current_frame = self.frames.current();

        for draw_buffer_index in 0..current_frame.draw_buffers_count() {
            current_frame.wait_for_draw_buffer_fence(self.logical_device()?, draw_buffer_index)?;
        }

        Ok(())
    }

    pub fn acquire_next_image(&self) -> crate::Result<(u32, bool)> {
        unsafe {
            let frame = self.frames.current();
            let result = self.swapchain()?.swapchain_loader.acquire_next_image(
                self.swapchain()?.swapchain,
                std::u64::MAX,
                frame.sync_primitives.present_complete_semaphore,
                Fence::null(),
            );

            match result {
                Err(ash::vk::Result::ERROR_OUT_OF_DATE_KHR | ash::vk::Result::SUBOPTIMAL_KHR) => {
                    Ok((0, true))
                }
                other => Ok(other?),
            }
        }
    }

    pub fn record_commands_to_draw_buffer<CommandOp>(
        &self,
        command_op: CommandOp,
    ) -> crate::Result<()>
    where
        CommandOp: FnOnce(&SetUpLogicalDevice, &SetUpCommandBufferWithFence) -> crate::Result<()>,
    {
        let logical_device = self.logical_device()?;

        let current_frame = self.frames.current();

        current_frame.reset_draw_buffer_fence(logical_device, 0)?;
        current_frame.reset_draw_buffer(logical_device, 0)?;
        current_frame.record_draw_buffer_commands(
            logical_device,
            0,
            |device, draw_command_buffer| command_op(device, draw_command_buffer),
        )?;

        Ok(())
    }

    pub fn record_render_pass<RenderPassOp>(
        &self,
        present_index: u32,
        draw_command_buffer: &SetUpCommandBufferWithFence,
        clear_values: &[ClearValue],
        mut render_pass_op: RenderPassOp,
    ) -> crate::Result<()>
    where
        RenderPassOp: FnMut() -> crate::Result<()>,
    {
        let logical_device = self.logical_device()?;

        let surface_resolution = self.surface_info.surface_extent()?;
        let render_pass_begin_info = RenderPassBeginInfo::builder()
            .render_pass(self.render_pass()?.render_pass)
            .framebuffer(self.framebuffers()?.framebuffers[present_index as usize])
            .render_area(surface_resolution.into())
            .clear_values(clear_values);

        unsafe {
            logical_device.cmd_begin_render_pass(
                draw_command_buffer.command_buffer,
                &render_pass_begin_info,
                SubpassContents::INLINE,
            );
            render_pass_op()?;
            logical_device.cmd_end_render_pass(draw_command_buffer.command_buffer);
        }

        Ok(())
    }

    pub fn submit_draw_buffers(&self) -> crate::Result<()> {
        let current_frame = self.frames.current();
        let logical_device = self.logical_device()?;

        for draw_buffer_index in 0..current_frame.draw_buffers_count() {
            current_frame.submit_draw_buffer_to_queue(
                logical_device,
                draw_buffer_index,
                logical_device.present_queue,
                &[PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
            )?;
        }
        Ok(())
    }

    pub fn present_image(&self, present_index: u32) -> crate::Result<bool> {
        let frame = self.frames.current();
        let wait_semaphores = [frame.sync_primitives.rendering_complete_semaphore];
        let swapchains = [self.swapchain()?.swapchain];
        let image_indices = [present_index];
        let present_info = PresentInfoKHR::builder()
            .wait_semaphores(&wait_semaphores) // &base.rendering_complete_semaphore)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        unsafe {
            let result = self
                .swapchain()?
                .swapchain_loader
                .queue_present(self.logical_device()?.present_queue, &present_info);
            match result {
                Err(ash::vk::Result::ERROR_OUT_OF_DATE_KHR | ash::vk::Result::SUBOPTIMAL_KHR) => {
                    Ok(true)
                }
                other => Ok(other?),
            }
        }
    }

    pub fn copy_data_to_buffer<T: Copy>(
        &self,
        buffer: &TypedAllocatedBuffer<T>,
        data: &[T],
    ) -> crate::Result<()> {
        let allocator = &mut self.lock_allocator()?;
        let logical_device = self.logical_device()?;
        let element_amount = buffer.element_amount;
        unsafe {
            let mut staging_buffer =
                SetUpStagingBuffer::allocate(logical_device, allocator, element_amount)?;
            staging_buffer.copy_from_slice(logical_device, data)?;
            self.submit_immediate_setup_commands(|device, setup_command_buffer| {
                buffer.copy_from_staging_buffer(device, &staging_buffer, setup_command_buffer)
            })?;
            staging_buffer.destroy(logical_device, allocator)?;
        }
        Ok(())
    }

    pub fn create_vertex_buffer<T: Copy>(
        &self,
        data: &[T],
    ) -> crate::Result<TypedAllocatedBuffer<T>> {
        unsafe {
            let data_amount = data.len();
            let buffer = TypedAllocatedBuffer::<T>::allocate_vertex_buffer(
                self.logical_device()?,
                &mut self.lock_allocator()?,
                data_amount,
            )?;
            self.copy_data_to_buffer(&buffer, data)?;
            Ok(buffer)
        }
    }

    pub fn create_index_buffer<T: Copy>(
        &self,
        data: &[T],
    ) -> crate::Result<TypedAllocatedBuffer<T>> {
        unsafe {
            let data_amount = data.len();
            let buffer = TypedAllocatedBuffer::<T>::allocate_index_buffer(
                self.logical_device()?,
                &mut self.lock_allocator()?,
                data_amount,
            )?;
            self.copy_data_to_buffer(&buffer, data)?;
            Ok(buffer)
        }
    }

    pub fn allocate_static_uniform_buffer<T>(&self) -> crate::Result<SetUpUniformBuffer<T>> {
        unsafe {
            SetUpUniformBuffer::<T>::allocate_static_uniform_buffer(
                self.logical_device()?,
                self.physical_device()?,
                &mut self.lock_allocator()?,
            )
        }
    }

    pub fn allocate_dynamic_uniform_buffer<T>(
        &self,
        count: usize,
    ) -> crate::Result<SetUpUniformBuffer<T>> {
        unsafe {
            SetUpUniformBuffer::<T>::allocate_dynamic_uniform_buffer(
                self.logical_device()?,
                self.physical_device()?,
                &mut self.lock_allocator()?,
                count,
            )
        }
    }

    pub fn allocate_storage_buffer<T>(&self, count: usize) -> crate::Result<SetUpStorageBuffer<T>> {
        unsafe {
            SetUpStorageBuffer::<T>::allocate(
                self.logical_device()?,
                &mut self.lock_allocator()?,
                count,
            )
        }
    }

    pub fn create_shader_of_stage(
        &self,
        compiled_bytes: Vec<u8>,
        stage: ShaderStageFlags,
    ) -> crate::Result<CompiledShaderModule> {
        unsafe {
            CompiledShaderModule::create_shader_of_stage(
                compiled_bytes,
                self.logical_device()?,
                stage,
            )
        }
    }

    pub fn create_vertex_shader(
        &self,
        compiled_bytes: Vec<u8>,
    ) -> crate::Result<CompiledShaderModule> {
        self.create_shader_of_stage(compiled_bytes, ShaderStageFlags::VERTEX)
    }

    pub fn create_fragment_shader(
        &self,
        compiled_bytes: Vec<u8>,
    ) -> crate::Result<CompiledShaderModule> {
        self.create_shader_of_stage(compiled_bytes, ShaderStageFlags::FRAGMENT)
    }

    pub fn create_texture<T>(
        &self,
        format: Format,
        texture_dimensions: Extent3D,
        data: &[T],
    ) -> crate::Result<Texture<T>>
    where
        T: Copy,
    {
        unsafe {
            let logical_device = self.logical_device()?;
            let mut texture = Texture::<T>::create(
                self.logical_device()?,
                &mut self.lock_allocator()?,
                format,
                texture_dimensions,
            )?;

            let element_amount =
                texture_dimensions.width * texture_dimensions.height * texture_dimensions.depth;
            let mut staging_buffer = SetUpStagingBuffer::allocate(
                logical_device,
                &mut self.lock_allocator()?,
                element_amount as usize,
            )?;
            staging_buffer.copy_from_slice(logical_device, data)?;

            self.submit_immediate_setup_commands(|device, setup_command_buffer| {
                texture.layout_transition_to_copy_target(device, setup_command_buffer);
                texture.copy_from_staging_buffer(device, &staging_buffer, setup_command_buffer)?;
                texture.layout_transition_to_shader_readable(device, setup_command_buffer);
                Ok(())
            })?;

            staging_buffer.destroy(logical_device, &mut self.lock_allocator()?)?;

            Ok(texture)
        }
    }

    pub unsafe fn create_sampler(
        &self,
        filter: Filter,
        sampler_address_mode: SamplerAddressMode,
    ) -> crate::Result<SetUpSampler> {
        unsafe { SetUpSampler::create(self.logical_device()?, filter, sampler_address_mode) }
    }

    pub fn wait_for_present_queue(&self) -> crate::Result<()> {
        unsafe {
            let logical_device = self.logical_device()?;
            logical_device.queue_wait_idle(logical_device.present_queue)?;
            Ok(())
        }
    }
}

impl<Verification: VerificationProvider + 'static> RenderContext<Verification> for VulkanContext {
    fn load(_: &mut Voxelar, window: &mut VoxelarWindow) -> crate::Result<Self>
    where
        Self: Sized,
    {
        unsafe {
            // App info
            let app_info = Self::create_app_info(&window);

            // Extension names
            let mut extension_names_raw =
                ash_window::enumerate_required_extensions(window.raw_display_handle())
                    .unwrap()
                    .to_vec();

            let verification_required_extensions = Verification::get_extensions();
            let verification_names_raw: Vec<*const c_char> =
                util::map_vec_ref(&verification_required_extensions, |name| name.as_ptr());
            extension_names_raw.extend(verification_names_raw);

            #[cfg(any(target_os = "macos", target_os = "ios"))]
            {
                extension_names_raw.push(KhrPortabilityEnumerationFn::name().as_ptr());
                // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
                extension_names_raw.push(KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
            }

            println!(
                "Extensions: {:?}",
                extension_names_raw
                    .iter()
                    .map(|ptr| CStr::from_ptr(*ptr).to_str().ok())
                    .collect::<Vec<Option<&str>>>()
            );

            // Layer names
            let verification_required_layers = Verification::get_layers();
            println!("Layers: {:?}", verification_required_layers);

            let layers_names_raw: Vec<*const c_char> =
                util::map_vec_ref(&verification_required_layers, |name| name.as_ptr());

            // Create flags
            let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
                InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
            } else {
                InstanceCreateFlags::default()
            };

            let create_info = InstanceCreateInfo::builder()
                .application_info(&app_info)
                .enabled_extension_names(&extension_names_raw)
                .enabled_layer_names(&layers_names_raw)
                .flags(create_flags);

            let entry = Entry::load()?;
            let instance: Instance = entry.create_instance(&create_info, None)?;

            let verification = Box::new(Verification::load(&entry, &instance)?);

            let surface_info = SetUpSurfaceInfo::create(&window, &entry, &instance)?;

            Ok(Self {
                entry,
                instance,
                surface_info,

                verification,

                last_creation_info: None,
                physical_device: None,
                logical_device: None,
                allocator: None,
                swapchain: None,
                present_images: None,
                command_pool_for_setup: None,
                depth_image: None,
                render_pass: None,
                framebuffers: None,

                frames: PerFrame::empty(),
            })
        }
    }

    fn get_info<T>(&self) -> crate::Result<T> {
        unimplemented!()
    }
}

impl Drop for VulkanContext {
    fn drop(&mut self) {
        if let Some(logical_device) = self.logical_device.as_ref() {
            logical_device.wait().unwrap();

            if let Some(render_pass) = self.render_pass.as_mut() {
                render_pass.destroy(&logical_device);
            }

            if let Some(framebuffers) = self.framebuffers.as_mut() {
                framebuffers.destroy(&logical_device);
            }

            if let Some(mut depth_image) = self.depth_image.take() {
                let mut allocator = self.lock_allocator().expect("No allocator defined");
                depth_image
                    .destroy(&logical_device, &mut allocator)
                    .expect("Failed to destroy depth image");
            }

            if let Some(command_pool_for_setup) = self.command_pool_for_setup.as_mut() {
                command_pool_for_setup.destroy(&logical_device);
            }

            if let Some(present_images) = self.present_images.as_mut() {
                present_images.destroy(&logical_device);
            }

            for frame in self.frames.iter_mut() {
                frame.destroy(&logical_device);
            }

            if let Some(swapchain) = self.swapchain.as_mut() {
                swapchain.destroy();
            }

            if let Some(allocator) = &mut self.allocator {
                unsafe {
                    ManuallyDrop::drop(allocator);
                }
            }
        }

        if let Some(logical_device) = self.logical_device.as_mut() {
            logical_device.destroy();
        }

        unsafe {
            self.surface_info.destroy();
            self.verification.destroy();
            self.instance.destroy_instance(None);
        }
    }
}
