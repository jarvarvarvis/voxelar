use ash::vk::Filter;
use ash::vk::{Sampler, SamplerAddressMode, SamplerCreateInfo};

use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpSampler {
    pub sampler: Sampler,
}

impl SetUpSampler {
    pub unsafe fn create(
        virtual_device: &SetUpVirtualDevice,
        filter: Filter,
        sampler_address_mode: SamplerAddressMode,
    ) -> crate::Result<Self> {
        let sampler_create_info = SamplerCreateInfo::builder()
            .mag_filter(filter)
            .min_filter(filter)
            .address_mode_u(sampler_address_mode)
            .address_mode_v(sampler_address_mode)
            .address_mode_w(sampler_address_mode);

        let sampler = virtual_device
            .device
            .create_sampler(&sampler_create_info, None)?;

        Ok(Self { sampler })
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        unsafe {
            virtual_device.device.destroy_sampler(self.sampler, None);
        }
    }
}
