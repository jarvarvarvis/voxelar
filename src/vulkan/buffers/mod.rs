//! This modules provides all abstractions for Vulkan buffers
//!
//! Module overview:
//! - aligned\_buffer: Provides an abstraction for buffers with custom alignments
//! - buffer: Provides an abstraction for GPU memory-allocated buffers
//! - staging\_buffer: Provides an abstraction for staging buffers (used when transferring data from CPU- to GPU-only memory)
//! - storage\_buffer: Provides an abstraction for shader storage buffers
//! - typed\_buffer: Provides an abstraction for buffers that hold data of a specific type
//! - uniform\_buffer: Provides an abstraction for uniform buffers

pub mod aligned_buffer;
pub mod buffer;
pub mod staging_buffer;
pub mod storage_buffer;
pub mod typed_buffer;
pub mod uniform_buffer;
