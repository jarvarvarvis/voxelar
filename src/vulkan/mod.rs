use std::ffi::{c_char, CStr, CString};

use paste::paste;

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;
use ash::vk;
use ash::vk::ApplicationInfo;
use ash::vk::ClearValue;
use ash::vk::PipelineStageFlags;
use ash::vk::ShaderStageFlags;
use ash::vk::SubpassContents;
use ash::vk::SurfaceKHR;
use ash::vk::{InstanceCreateFlags, InstanceCreateInfo};
use ash::{Entry, Instance};

#[cfg(any(target_os = "macos", target_os = "ios"))]
use ash::vk::{KhrGetPhysicalDeviceProperties2Fn, KhrPortabilityEnumerationFn};

pub mod buffer;
pub mod command;
pub mod command_buffer;
pub mod creation_info;
pub mod debug;
pub mod depth_image;
pub mod descriptor_set_logic;
pub mod descriptor_set_logic_builder;
pub mod descriptor_set_layout;
pub mod descriptor_set_layout_builder;
pub mod frame_data;
pub mod framebuffers;
pub mod graphics_pipeline_builder;
pub mod physical_device;
pub mod pipeline_layout;
pub mod pipeline_layout_builder;
pub mod present_images;
pub mod render_pass;
pub mod shader;
pub mod swapchain;
pub mod sync;
pub mod util;
pub mod virtual_device;

use crate::render_context::RenderContext;
use crate::result::Context;
use crate::window::VoxelarWindow;
use crate::Voxelar;

use crate::engine::per_frame::PerFrame;

use self::buffer::AllocatedBuffer;
use self::command::SetUpCommandLogic;
use self::command_buffer::SetUpCommandBufferWithFence;
use self::creation_info::DataStructureCreationInfo;
use self::creation_info::PresentModeInitMode;
use self::debug::VerificationProvider;
use self::depth_image::SetUpDepthImage;
use self::frame_data::FrameData;
use self::framebuffers::SetUpFramebuffers;
use self::physical_device::SetUpPhysicalDevice;
use self::present_images::SetUpPresentImages;
use self::render_pass::SetUpRenderPass;
use self::shader::CompiledShaderModule;
use self::swapchain::SetUpSwapchain;
use self::virtual_device::SetUpVirtualDevice;

pub struct VulkanContext<Verification: VerificationProvider> {
    pub entry: Entry,
    pub instance: Instance,

    pub verification: Verification,

    pub surface_loader: Surface,
    pub surface: SurfaceKHR,

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

impl<Verification: VerificationProvider> VulkanContext<Verification> {
    fn create_app_info(window: &VoxelarWindow) -> ApplicationInfo {
        let app_name = CString::new(window.title()).unwrap();

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
                &self.surface_loader,
                self.surface,
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

    pub fn create_swapchain(
        &mut self,
        window_size: (i32, i32),
        present_mode_init_mode: PresentModeInitMode,
    ) -> crate::Result<()> {
        unsafe {
            self.swapchain = Some(SetUpSwapchain::create_with_defaults(
                &self.instance,
                &self.surface_loader,
                self.surface,
                self.physical_device()?,
                self.virtual_device()?,
                window_size.0 as u32,
                window_size.1 as u32,
                present_mode_init_mode,
            )?);
        }

        Ok(())
    }

    pub fn create_present_images(&mut self) -> crate::Result<()> {
        unsafe {
            self.present_images = Some(SetUpPresentImages::create_with_defaults(
                self.physical_device()?,
                self.virtual_device()?,
                self.swapchain()?,
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

    pub fn create_depth_image(&mut self, window_size: (i32, i32)) -> crate::Result<()> {
        unsafe {
            self.depth_image = Some(SetUpDepthImage::create_with_defaults(
                self.physical_device()?,
                self.virtual_device()?,
                window_size.0 as u32,
                window_size.1 as u32,
            )?);

            let depth_image = self.depth_image()?;
            self.submit_setup_command(|device, setup_command_buffer| {
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
                self.swapchain()?,
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
                self.physical_device()?,
            )?);
        }

        Ok(())
    }

    pub fn create_per_frame_data(&mut self, frame_overlap: usize) -> crate::Result<()> {
        unsafe {
            let virtual_device = self.virtual_device()?;
            self.frames = PerFrame::try_init(
                || FrameData::create_with_defaults(virtual_device),
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
        self.create_virtual_device()?;
        self.create_swapchain(window_size, creation_info.swapchain_present_mode)?;
        self.create_present_images()?;
        self.create_command_logic_for_setup()?;
        self.create_depth_image(window_size)?;
        self.create_render_pass()?;
        self.create_framebuffers()?;
        self.create_per_frame_data(creation_info.frame_overlap)?;
        Ok(())
    }
}

impl<Verification: VerificationProvider> VulkanContext<Verification> {
    pub fn submit_setup_command<F>(&self, command_buffer_op: F) -> crate::Result<()>
    where
        F: FnOnce(&SetUpVirtualDevice, &SetUpCommandBufferWithFence) -> crate::Result<()>,
    {
        let setup_command_buffer = self.command_logic_for_setup()?.get_command_buffer(0);
        let present_queue = self.virtual_device()?.present_queue;
        setup_command_buffer.submit(
            self.virtual_device()?,
            present_queue,
            &[],
            &[],
            &[],
            command_buffer_op,
        )
    }

    pub fn select_frame(&mut self, current_frame_index: usize) {
        self.frames.select(current_frame_index);
    }

    pub fn submit_render_pass_command<F>(
        &self,
        present_index: u32,
        clear_values: &[ClearValue],
        command_buffer_op: F,
    ) -> crate::Result<()>
    where
        F: FnOnce(&SetUpVirtualDevice, &SetUpCommandBufferWithFence) -> crate::Result<()>,
    {
        let surface_resolution = self.swapchain()?.surface_extent;
        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(self.render_pass()?.render_pass)
            .framebuffer(self.framebuffers()?.framebuffers[present_index as usize])
            .render_area(surface_resolution.into())
            .clear_values(clear_values);

        let current_frame = self.frames.current();
        let present_queue = self.virtual_device()?.present_queue;

        current_frame.submit_to_draw_buffer(
            self.virtual_device()?,
            present_queue,
            &[PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
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

        Ok(())
    }

    pub fn frame_overlap(&self) -> u32 {
        self.frames.len() as u32
    }

    pub fn acquire_next_image(&self) -> crate::Result<(u32, bool)> {
        unsafe {
            let frame = self.frames.current();
            let present_index_and_success = self.swapchain()?.swapchain_loader.acquire_next_image(
                self.swapchain()?.swapchain,
                std::u64::MAX,
                frame.sync_primitives.present_complete_semaphore,
                vk::Fence::null(),
            )?;
            Ok(present_index_and_success)
        }
    }

    pub fn present_image(&self, present_index: u32) -> crate::Result<()> {
        let frame = self.frames.current();
        let wait_semaphores = [frame.sync_primitives.rendering_complete_semaphore];
        let swapchains = [self.swapchain()?.swapchain];
        let image_indices = [present_index];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&wait_semaphores) // &base.rendering_complete_semaphore)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        unsafe {
            self.swapchain()?
                .swapchain_loader
                .queue_present(self.virtual_device()?.present_queue, &present_info)?;
            Ok(())
        }
    }

    pub fn create_vertex_buffer<T: Copy>(&self, data: &[T]) -> crate::Result<AllocatedBuffer<T>> {
        unsafe {
            AllocatedBuffer::<T>::create_vertex_buffer(
                self.virtual_device()?,
                self.physical_device()?,
                data,
            )
        }
    }

    pub fn create_index_buffer(&self, data: &[u32]) -> crate::Result<AllocatedBuffer<u32>> {
        unsafe {
            AllocatedBuffer::<u32>::create_index_buffer(
                self.virtual_device()?,
                self.physical_device()?,
                data,
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

impl<Verification: VerificationProvider> RenderContext for VulkanContext<Verification> {
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

            let verification = Verification::load(&entry, &instance)?;

            let surface = ash_window::create_surface(
                &entry,
                &instance,
                window.raw_display_handle(),
                window.raw_window_handle(),
                None,
            )?;

            let surface_loader = Surface::new(&entry, &instance);

            Ok(Self {
                entry,
                instance,
                verification,
                surface_loader,
                surface,

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

    fn get_info(&self) -> crate::Result<String> {
        todo!()
    }
}

impl<Verification: VerificationProvider> Drop for VulkanContext<Verification> {
    fn drop(&mut self) {
        unsafe {
            if let Some(virtual_device) = self.virtual_device.as_mut() {
                virtual_device.wait();

                if let Some(render_pass) = self.render_pass.as_mut() {
                    render_pass.destroy(&virtual_device);
                }

                if let Some(framebuffers) = self.framebuffers.as_mut() {
                    framebuffers.destroy(&virtual_device);
                }

                if let Some(depth_image) = self.depth_image.as_mut() {
                    depth_image.destroy(&virtual_device);
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

                virtual_device.destroy();
            }
            self.surface_loader.destroy_surface(self.surface, None);
            self.verification.destroy();
            self.instance.destroy_instance(None);
        }
    }
}
