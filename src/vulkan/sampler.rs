use ash::vk::Filter;
use ash::vk::{Sampler, SamplerAddressMode, SamplerCreateInfo};

use super::logical_device::SetUpLogicalDevice;

pub struct SetUpSampler {
    pub sampler: Sampler,
}

impl SetUpSampler {
    pub unsafe fn create(
        logical_device: &SetUpLogicalDevice,
        filter: Filter,
        sampler_address_mode: SamplerAddressMode,
    ) -> crate::Result<Self> {
        let sampler_create_info = SamplerCreateInfo::builder()
            .mag_filter(filter)
            .min_filter(filter)
            .address_mode_u(sampler_address_mode)
            .address_mode_v(sampler_address_mode)
            .address_mode_w(sampler_address_mode);

        let sampler = logical_device.create_sampler(&sampler_create_info, None)?;

        Ok(Self { sampler })
    }

    pub fn destroy(&mut self, logical_device: &SetUpLogicalDevice) {
        unsafe {
            logical_device.destroy_sampler(self.sampler, None);
        }
    }
}
