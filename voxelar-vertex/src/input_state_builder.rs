use std::marker::PhantomData;

use super::*;

pub struct KeepAlive<'other, T, Other> {
    value: T,
    phantom: PhantomData<&'other Other>,
}

impl<'other, T, Other> KeepAlive<'other, T, Other> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}

impl<'other, T, Other> std::ops::Deref for KeepAlive<'other, T, Other> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub struct VertexInputStateBuilder<'builder> {
    pub vertex_input_binding_descriptions: Vec<VertexInputBindingDescription>,
    pub vertex_input_attribute_descriptions: Vec<VertexInputAttributeDescription>,
    phantom: PhantomData<&'builder ()>,
}

impl<'builder> VertexInputStateBuilder<'builder> {
    pub fn new() -> Self {
        Self {
            vertex_input_binding_descriptions: vec![],
            vertex_input_attribute_descriptions: vec![],
            phantom: PhantomData,
        }
    }

    pub fn add_data_from_type<T: VertexInput>(mut self) -> Self {
        let data = T::input_state_info();
        self.vertex_input_binding_descriptions
            .extend(data.vertex_input_binding_descriptions);
        self.vertex_input_attribute_descriptions
            .extend(data.vertex_input_attribute_descriptions);
        self
    }

    pub fn build(&'builder self) -> KeepAlive<'builder, PipelineVertexInputStateCreateInfo, Self> {
        let vertex_input_state_info = PipelineVertexInputStateCreateInfo::builder()
            .vertex_attribute_descriptions(&self.vertex_input_attribute_descriptions)
            .vertex_binding_descriptions(&self.vertex_input_binding_descriptions)
            .build();
        KeepAlive::new(vertex_input_state_info)
    }
}
