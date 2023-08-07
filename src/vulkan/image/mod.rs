//! This is a module that provides all abstractions and functionality related to images, e.g.
//! allocation on the GPU, loading an image from a file using the `image` crate, etc.
//!
//! - image: Provides an abstraction for GPU memory-allocated images
//! - image\_view: Provides an abstraction for Vulkan image views
//! - sampler: Provides a wrapper around image samplers
//! - texture: Provides an abstraction for GPU-allocated textures
//! - typed\_image: Provides an abstraction for images that hold data of a specific type

pub mod image;
pub mod image_view;
pub mod sampler;
pub mod texture;
pub mod typed_image;

pub use image_crate::*;

