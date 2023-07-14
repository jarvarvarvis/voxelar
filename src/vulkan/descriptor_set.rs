use ash::vk::DescriptorSet;

pub struct SetUpDescriptorSet {
    pub descriptor_set: DescriptorSet,
}

impl SetUpDescriptorSet {
    pub unsafe fn create(
        descriptor_set: DescriptorSet,
    ) -> crate::Result<Self> {
        Ok(Self {
            descriptor_set
        })
    }
}
