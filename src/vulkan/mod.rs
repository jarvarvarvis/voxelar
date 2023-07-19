use std::ffi::{c_char, CStr, CString};

use paste::paste;

use ash::extensions::ext::DebugUtils;
use ash::vk;
use ash::vk::ApplicationInfo;
use ash::vk::ClearValue;
use ash::vk::Extent2D;
use ash::vk::PipelineStageFlags;
use ash::vk::ShaderStageFlags;
use ash::vk::SubpassContents;
use ash::vk::{InstanceCreateFlags, InstanceCreateInfo};
use ash::{Entry, Instance};

#[cfg(any(target_os = "macos", target_os = "ios"))]
use ash::vk::{KhrGetPhysicalDeviceProperties2Fn, KhrPortabilityEnumerationFn};

pub mod allocator;
pub mod buffer;
pub mod command;
pub mod command_buffer;
pub mod creation_info;
pub mod debug;
pub mod dedicated_pool_allocator;
pub mod depth_image;
pub mod descriptor_set_layout;
pub mod descriptor_set_layout_builder;
pub mod descriptor_set_logic;
pub mod descriptor_set_logic_builder;
pub mod descriptor_set_update_builder;
pub mod dynamic_descriptor_buffer;
pub mod frame_data;
pub mod framebuffers;
pub mod graphics_pipeline_builder;
pub mod memory_range;
pub mod naive_allocator;
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
pub mod typed_buffer;
pub mod util;
pub mod virtual_device;

#[cfg(test)]
pub mod test_context;

use crate::render_context::RenderContext;
use crate::result::Context;
use crate::window::VoxelarWindow;
use crate::Voxelar;

use crate::vulkan::per_frame::PerFrame;

use self::allocator::Allocator;
use self::command::SetUpCommandLogic;
use self::command_buffer::SetUpCommandBufferWithFence;
use self::creation_info::DataStructureCreationInfo;
use self::creation_info::PresentModeInitMode;
use self::debug::VerificationProvider;
use self::depth_image::SetUpDepthImage;
use self::dynamic_descriptor_buffer::DynamicDescriptorBuffer;
use self::frame_data::FrameData;
use self::framebuffers::SetUpFramebuffers;
use self::physical_device::SetUpPhysicalDevice;
use self::present_images::SetUpPresentImages;
use self::render_pass::SetUpRenderPass;
use self::shader::CompiledShaderModule;
use self::surface::SetUpSurfaceInfo;
use self::swapchain::SetUpSwapchain;
use self::typed_buffer::TypedAllocatedBuffer;
use self::virtual_device::SetUpVirtualDevice;

pub struct VulkanContext {
    pub entry: Entry,
    pub instance: Instance,
    pub surface_info: SetUpSurfaceInfo,

    pub verification: Box<dyn VerificationProvider>,
    pub allocator: Box<dyn Allocator>,

    pub last_creation_info: Option<DataStructureCreationInfo>,
    pub physical_device: Option<SetUpPhysicalDevice>,
    pub virtual_device: Option<SetUpVirtualDevice>,
    pub swapchain: Option<SetUpSwapchain>,
    pub present_images: Option<SetUpPresentImages>,
    pub command_logic_for_setup: Option<SetUpCommandLogic>,
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
            .api_version(vk::make_api_version(0, 1, 0, 0));

        *app_info
    }

    generate_safe_getter!(
        physical_device,
        SetUpPhysicalDevice,
        "No physical device was set up yet! Use VulkanContext::find_usable_physical_device to do so"
    );

    generate_safe_getter!(
        virtual_device,
        SetUpVirtualDevice,
        "No virtual device was set up yet! Use VulkanContext::create_virtual_device to do so"
    );

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
        command_logic_for_setup,
        SetUpCommandLogic,
        "No command logic for setup commands was set up yet! Use VulkanContext::create_command_logic_for_setup to do so"
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

    pub fn create_virtual_device(&mut self) -> crate::Result<()> {
        unsafe {
            self.virtual_device = Some(SetUpVirtualDevice::create_with_defaults(
                &self.instance,
                self.physical_device()?,
            )?);
        }

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
                self.virtual_device()?,
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
                self.virtual_device()?,
                self.swapchain()?,
                &self.surface_info,
            )?);
        }

        Ok(())
    }

    pub fn create_command_logic_for_setup(&mut self) -> crate::Result<()> {
        unsafe {
            self.command_logic_for_setup = Some(SetUpCommandLogic::create_with_one_primary_buffer(
                self.virtual_device()?,
            )?);
            Ok(())
        }
    }

    pub fn create_depth_image(&mut self) -> crate::Result<()> {
        unsafe {
            self.depth_image = Some(SetUpDepthImage::create_with_defaults(
                self.physical_device()?,
                self.virtual_device()?,
                self.allocator.as_ref(),
                &self.surface_info,
            )?);

            let depth_image = self.depth_image()?;
            self.submit_immediate_setup_commands(|device, setup_command_buffer| {
                depth_image
                    .submit_pipeline_barrier_command(device, setup_command_buffer.command_buffer);
                Ok(())
            })?;
        }
        Ok(())
    }

    pub fn create_framebuffers(&mut self) -> crate::Result<()> {
        unsafe {
            self.framebuffers = Some(SetUpFramebuffers::create(
                self.virtual_device()?,
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
                self.virtual_device()?,
                &self.surface_info,
            )?);
        }

        Ok(())
    }

    pub fn create_per_frame_data(&mut self, frame_overlap: usize) -> crate::Result<()> {
        unsafe {
            let virtual_device = self.virtual_device()?;
            self.frames = PerFrame::try_init(
                |_| FrameData::create_with_defaults(virtual_device),
                frame_overlap,
            )?;
            Ok(())
        }
    }

    pub fn create_default_data_structures(
        &mut self,
        window_size: (i32, i32),
        creation_info: DataStructureCreationInfo,
    ) -> crate::Result<()> {
        self.find_usable_physical_device()?;
        if let Some(physical_device) = &self.physical_device {
            self.surface_info.update(physical_device, window_size)?;
        }
        self.create_virtual_device()?;
        self.allocator
            .setup(self.virtual_device()?, self.physical_device()?)?;
        self.create_swapchain(creation_info.swapchain_present_mode)?;
        self.create_present_images()?;
        self.create_command_logic_for_setup()?;
        self.create_depth_image()?;
        self.create_render_pass()?;
        self.create_framebuffers()?;
        self.create_per_frame_data(creation_info.frame_overlap)?;
        self.last_creation_info = Some(creation_info);

        Ok(())
    }

    pub fn recreate_swapchain_and_related_data_structures_with_size(
        &mut self,
        window_size: (i32, i32),
    ) -> crate::Result<()> {
        let creation_info = self.last_creation_info.context(
            "No last creation info was set, so data structures can't be recreated".to_string(),
        )?;

        if let Some(physical_device) = &self.physical_device {
            self.surface_info.update(physical_device, window_size)?;
        }

        let new_swapchain = self.create_new_swapchain(creation_info.swapchain_present_mode)?;

        if let Some(virtual_device) = self.virtual_device.as_mut() {
            virtual_device.wait();

            if let Some(render_pass) = self.render_pass.as_mut() {
                render_pass.destroy(&virtual_device);
            }

            if let Some(framebuffers) = self.framebuffers.as_mut() {
                framebuffers.destroy(&virtual_device);
            }

            if let Some(depth_image) = self.depth_image.as_mut() {
                depth_image.destroy(&virtual_device, self.allocator.as_ref());
            }

            if let Some(command_logic_for_setup) = self.command_logic_for_setup.as_mut() {
                command_logic_for_setup.destroy(&virtual_device);
            }

            if let Some(present_images) = self.present_images.as_mut() {
                present_images.destroy(&virtual_device);
            }

            for frame in self.frames.iter_mut() {
                frame.destroy(&virtual_device);
            }

            if let Some(swapchain) = self.swapchain.as_mut() {
                swapchain.destroy();
            }
        }

        self.swapchain = Some(new_swapchain);
        self.create_present_images()?;
        self.create_command_logic_for_setup()?;
        self.create_depth_image()?;
        self.create_render_pass()?;
        self.create_framebuffers()?;
        self.create_per_frame_data(creation_info.frame_overlap)?;

        Ok(())
    }
}

impl VulkanContext {
    pub fn get_surface_extent(&self) -> crate::Result<Extent2D> {
        self.surface_info.surface_extent()
    }

    pub fn submit_immediate_setup_commands<F>(&self, command_buffer_op: F) -> crate::Result<()>
    where
        F: FnOnce(&SetUpVirtualDevice, &SetUpCommandBufferWithFence) -> crate::Result<()>,
    {
        let setup_command_buffer = self.command_logic_for_setup()?.get_command_buffer(0);
        let virtual_device = self.virtual_device()?;
        let present_queue = virtual_device.present_queue;
        setup_command_buffer.wait_for_fence(virtual_device)?;
        setup_command_buffer.reset(virtual_device)?;
        setup_command_buffer.record_commands(virtual_device, command_buffer_op)?;
        setup_command_buffer.submit(self.virtual_device()?, present_queue, &[], &[], &[])
    }

    pub fn select_frame(&mut self, current_frame_index: usize) {
        self.frames.select(current_frame_index);
    }

    pub fn submit_immediate_render_pass_commands<F>(
        &self,
        present_index: u32,
        clear_values: &[ClearValue],
        command_buffer_op: F,
    ) -> crate::Result<()>
    where
        F: FnOnce(&SetUpVirtualDevice, &SetUpCommandBufferWithFence) -> crate::Result<()>,
    {
        let virtual_device = self.virtual_device()?;

        let surface_resolution = self.surface_info.surface_extent()?;
        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(self.render_pass()?.render_pass)
            .framebuffer(self.framebuffers()?.framebuffers[present_index as usize])
            .render_area(surface_resolution.into())
            .clear_values(clear_values);

        let current_frame = self.frames.current();
        let present_queue = virtual_device.present_queue;

        current_frame.wait_for_draw_buffer_fence(virtual_device)?;
        current_frame.reset_draw_buffer(virtual_device)?;
        current_frame.record_draw_buffer_commands(
            virtual_device,
            |device, draw_command_buffer| {
                let vk_device = &device.device;
                unsafe {
                    vk_device.cmd_begin_render_pass(
                        draw_command_buffer.command_buffer,
                        &render_pass_begin_info,
                        SubpassContents::INLINE,
                    );
                    command_buffer_op(device, draw_command_buffer)?;
                    vk_device.cmd_end_render_pass(draw_command_buffer.command_buffer);
                    Ok(())
                }
            },
        )?;
        current_frame.submit_draw_buffer(
            self.virtual_device()?,
            present_queue,
            &[PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
        )?;

        Ok(())
    }

    pub fn frame_overlap(&self) -> usize {
        self.frames.len()
    }

    pub fn acquire_next_image(&self) -> crate::Result<(u32, bool)> {
        unsafe {
            let frame = self.frames.current();
            let result = self.swapchain()?.swapchain_loader.acquire_next_image(
                self.swapchain()?.swapchain,
                std::u64::MAX,
                frame.sync_primitives.present_complete_semaphore,
                vk::Fence::null(),
            );

            match result {
                Err(ash::vk::Result::ERROR_OUT_OF_DATE_KHR | ash::vk::Result::SUBOPTIMAL_KHR) => {
                    Ok((0, true))
                }
                other => Ok(other?),
            }
        }
    }

    pub fn present_image(&self, present_index: u32) -> crate::Result<bool> {
        let frame = self.frames.current();
        let wait_semaphores = [frame.sync_primitives.rendering_complete_semaphore];
        let swapchains = [self.swapchain()?.swapchain];
        let image_indices = [present_index];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&wait_semaphores) // &base.rendering_complete_semaphore)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        unsafe {
            let result = self
                .swapchain()?
                .swapchain_loader
                .queue_present(self.virtual_device()?.present_queue, &present_info);
            match result {
                Err(ash::vk::Result::ERROR_OUT_OF_DATE_KHR | ash::vk::Result::SUBOPTIMAL_KHR) => {
                    Ok(true)
                }
                other => Ok(other?),
            }
        }
    }

    pub fn create_vertex_buffer<T: Copy>(
        &self,
        data: &[T],
    ) -> crate::Result<TypedAllocatedBuffer<T>> {
        unsafe {
            TypedAllocatedBuffer::<T>::create_vertex_buffer(
                self.virtual_device()?,
                self.physical_device()?,
                self.allocator.as_ref(),
                data,
            )
        }
    }

    pub fn create_index_buffer<T: Copy>(
        &self,
        data: &[T],
    ) -> crate::Result<TypedAllocatedBuffer<T>> {
        unsafe {
            TypedAllocatedBuffer::<T>::create_index_buffer(
                self.virtual_device()?,
                self.physical_device()?,
                self.allocator.as_ref(),
                data,
            )
        }
    }

    pub fn allocate_dynamic_descriptor_uniform_buffer<T>(
        &self,
        count: usize,
    ) -> crate::Result<DynamicDescriptorBuffer<T>> {
        unsafe {
            DynamicDescriptorBuffer::<T>::allocate_uniform_buffer(
                self.virtual_device()?,
                self.physical_device()?,
                count,
                self.allocator.as_ref(),
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
                self.virtual_device()?,
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
}

impl<Alloc: Allocator + 'static, Verification: VerificationProvider + 'static>
    RenderContext<(Alloc, Verification)> for VulkanContext
{
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
            extension_names_raw.push(DebugUtils::name().as_ptr());

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
            let layer_names = Verification::get_layers();
            println!("Layers: {:?}", layer_names);

            let layers_names_raw: Vec<*const c_char> =
                util::map_vec_ref(&layer_names, |name| name.as_ptr());

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

            let allocator = Box::new(Alloc::new());
            let verification = Box::new(Verification::load(&entry, &instance)?);

            let surface_info = SetUpSurfaceInfo::create(&window, &entry, &instance)?;

            Ok(Self {
                entry,
                instance,
                surface_info,

                verification,
                allocator,

                last_creation_info: None,
                physical_device: None,
                virtual_device: None,
                swapchain: None,
                present_images: None,
                command_logic_for_setup: None,
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
        if let Some(virtual_device) = self.virtual_device.as_mut() {
            virtual_device.wait();

            if let Some(render_pass) = self.render_pass.as_mut() {
                render_pass.destroy(&virtual_device);
            }

            if let Some(framebuffers) = self.framebuffers.as_mut() {
                framebuffers.destroy(&virtual_device);
            }

            if let Some(depth_image) = self.depth_image.as_mut() {
                depth_image.destroy(&virtual_device, self.allocator.as_ref());
            }

            if let Some(command_logic_for_setup) = self.command_logic_for_setup.as_mut() {
                command_logic_for_setup.destroy(&virtual_device);
            }

            if let Some(present_images) = self.present_images.as_mut() {
                present_images.destroy(&virtual_device);
            }

            for frame in self.frames.iter_mut() {
                frame.destroy(&virtual_device);
            }

            if let Some(swapchain) = self.swapchain.as_mut() {
                swapchain.destroy();
            }

            self.allocator.destroy(virtual_device);

            virtual_device.destroy();
        }

        unsafe {
            self.surface_info.destroy();
            self.verification.destroy();
            self.instance.destroy_instance(None);
        }
    }
}
