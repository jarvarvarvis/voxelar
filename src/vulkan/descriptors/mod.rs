//! This is a module that provides all descriptor-related functionality.
//! There are abstractions for building descriptor sets, attaching descriptors to them etc.
//!
//! - descriptor\_set\_layout: Provides a wrapper around `DescriptorSetLayout`s
//! - descriptor\_set\_layout\_builder: Provides an abstraction for building `(SetUp)DescriptorSetLayout`s
//! - descriptor\_set\_logic: Provides an abstraction for `DescriptorSet` allocation
//! - descriptor\_set\_logic\_builder: Provides an abstraction for building `SetUpDescriptorSetLogic`s
//! - descriptor\_set\_update\_builder: Provides an abstraction for updating descriptor sets and specifying attached descriptors

pub mod descriptor_set_layout;
pub mod descriptor_set_layout_builder;
pub mod descriptor_set_logic;
pub mod descriptor_set_logic_builder;
pub mod descriptor_set_update_builder;
