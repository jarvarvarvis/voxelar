//! This is a module that contains the `AsAny` trait which is useful when you want to convert a
//! reference to a type to a reference of `dyn std::any::Any`.
//!
//! Currently, this trait is only used by the tests for the `DedicatedPoolAllocator`.

use std::any::Any;

/// The `AsAny` trait.
pub trait AsAny: 'static {
    /// This function converts a reference of this type to a reference of `dyn std::any::Any`.
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
