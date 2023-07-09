use std::borrow::Cow;
use std::ffi::CStr;

use ash::extensions::ext::DebugUtils;
use ash::vk::{self, DebugUtilsMessengerEXT};
use ash::vk::{
    DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
    DebugUtilsMessengerCallbackDataEXT,
};
use ash::{Entry, Instance};

pub trait VerificationProvider {
    fn get_layers<'a>() -> Vec<&'a CStr>;
    fn load(entry: &Entry, instance: &Instance) -> crate::Result<Self>
    where
        Self: Sized;
    fn destroy(&mut self);
}

pub struct NoVerification;

impl VerificationProvider for NoVerification {
    fn get_layers<'a>() -> Vec<&'a CStr> {
        vec![]
    }

    fn load(_: &Entry, _: &Instance) -> crate::Result<Self> {
        Ok(Self)
    }

    fn destroy(&mut self) {}
}

pub unsafe extern "system" fn vulkan_debug_callback(
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_type: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{message_severity:?}: [{message_type:?}: {message_id_name} ({message_id_number})]: {message}",
    );

    vk::FALSE
}

pub struct KHRVerificationAndDebugMessenger {
    debug_utils_loader: DebugUtils,
    debug_messenger: DebugUtilsMessengerEXT,
}

impl VerificationProvider for KHRVerificationAndDebugMessenger {
    fn get_layers<'a>() -> Vec<&'a CStr> {
        unsafe {
            vec![CStr::from_bytes_with_nul_unchecked(
                b"VK_LAYER_KHRONOS_validation\0",
            )]
        }
    }

    fn load(entry: &Entry, instance: &Instance) -> crate::Result<Self> {
        unsafe {
            let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
                .message_severity(
                    vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                        | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
                )
                .message_type(
                    vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                )
                .pfn_user_callback(Some(vulkan_debug_callback));

            let debug_utils_loader = DebugUtils::new(&entry, &instance);
            let debug_messenger =
                debug_utils_loader.create_debug_utils_messenger(&debug_info, None)?;

            Ok(Self {
                debug_utils_loader,
                debug_messenger,
            })
        }
    }

    fn destroy(&mut self) {
        unsafe {
            self.debug_utils_loader
                .destroy_debug_utils_messenger(self.debug_messenger, None);
        }
    }
}
