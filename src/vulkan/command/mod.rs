//! This is a module that provides all functionality and abstractions for using Vulkan commands,
//! namely command buffers into which commands are recorded, as well as command pools from which
//! the command buffers are allocated.
//!
//! - command\_buffer: Provides an abstraction for command buffers and access synchronization
//! - command\_pool: Provides an abstraction for command buffer allocation

pub mod command_buffer;
pub mod command_pool;
