//! This is a module that provides voxelar's generic error type (`VoxelarError`) and a `Result<T>`
//! type that uses this error type.
//!
//! This module provides three related macros:
//! - error: A macro that wraps a formatted string in the error variant `VoxelarError::Custom`
//! - bail: A macro that wraps a formatted string in the error variant `VoxelarError::Custom`
//!         and returns this value wrapped in a `Result`
//! - verify: A macro that wraps a formatted string in the error variant `VoxelarError::Custom`
//!           and returns this value wrapped in a `Result` only if the specified condition is
//!           truthy
//!
//! Additionally, this module provides a `Context` trait and implementations for `Option` and
//! `std::result::Result<T, E: std::error::Error>`.
//! This trait can be used to convert values into the `Result<T>` of this module, while attaching
//! some further information.
//!
//! # Examples
//!
//! Simple usage:
//! ```
//! use voxelar::*;
//!
//! fn get_value() -> Result<i32> {
//!     Ok(12)
//! }
//!
//! fn main() -> Result<()> {
//!     let value = get_value()?;
//!     assert_eq!(12, value);
//!     Ok(())
//! }
//! ```

use std::ffi::NulError;
use std::str::Utf8Error;

/// Voxelar's generic error type with variants for various kinds of errors.
#[derive(Debug)]
pub enum VoxelarError {
    /// This variant wraps any value that implements std::error::Error in a box.
    Wrapped(Box<dyn std::error::Error>),

    /// This variant wraps a String value that provides a custom error message.
    ///
    /// Used by the `error`, `bail` and `verify` macros, as well as the `Context` implementations
    /// for `Option` and `std::result::Result`.
    Custom(String),

    /// This variant wraps an `std::ffi::NulError` that can occur as a result of the `CString::new`
    /// method when an interior nul byte was found.
    NulError(NulError),

    /// This variant wraps an `std::io::Error` that can occur on I/O operations.
    IOError(std::io::Error),

    /// This variant wraps an `std::str::Utf8Error` that can occur when attempting to interpret a
    /// sequence of bytes (`u8`) as a string.
    Utf8Error(Utf8Error),

    /// This variant wraps a `glfw::InitError` that can occur when `glfw::init` is called.
    GlfwInitError(glfw::InitError),

    /// This variant wraps an `ash::vk::Result` that can occur as the result of functions provided
    /// by the `ash` crate.
    VkError(ash::vk::Result),

    /// This variant wraps an `ash::LoadingError` that can occur when trying to load the Vulkan API
    /// using the `ash` crate.
    VkLoadingError(ash::LoadingError),
}

macro_rules! write_err {
    ($fmt:ident, $kind:ident, $err:ident) => {
        write!($fmt, "Voxelar error ({}): {}", stringify!($kind), $err)
    };
}

impl std::fmt::Display for VoxelarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoxelarError::Wrapped(err) => write_err!(f, Wrapped, err),
            VoxelarError::Custom(err) => write_err!(f, Custom, err),
            VoxelarError::NulError(err) => write_err!(f, NulError, err),
            VoxelarError::IOError(err) => write_err!(f, IOError, err),
            VoxelarError::Utf8Error(err) => write_err!(f, Utf8Error, err),
            VoxelarError::GlfwInitError(err) => write_err!(f, GlfwInitError, err),
            VoxelarError::VkError(err) => write_err!(f, VkError, err),
            VoxelarError::VkLoadingError(err) => write_err!(f, VkLoadingError, err),
        }
    }
}

macro_rules! error_impl_from {
    ($type:ty, $variant:expr) => {
        impl From<$type> for VoxelarError {
            fn from(value: $type) -> Self {
                $variant(value)
            }
        }
    };
}

error_impl_from!(Box<dyn std::error::Error>, Self::Wrapped);
error_impl_from!(String, Self::Custom);
error_impl_from!(std::io::Error, Self::IOError);
error_impl_from!(NulError, Self::NulError);
error_impl_from!(Utf8Error, Self::Utf8Error);
error_impl_from!(glfw::InitError, Self::GlfwInitError);
error_impl_from!(ash::vk::Result, Self::VkError);
error_impl_from!(ash::LoadingError, Self::VkLoadingError);

/// A type definition of `std::result::Result` that uses `VoxelarError` as the error type.
///
/// This should preferably be used instead of the written-out form.
pub type Result<T> = std::result::Result<T, VoxelarError>;

/// A macro that wraps a formatted string in the error variant `VoxelarError::Custom`.
///
/// This macro's arguments match against any amount of token trees, separated by commas.
/// This data is then passed to the `format` macro, from which the message will be created.
///
/// # Examples
///
/// ```
/// use voxelar::*;
/// use voxelar::result::*;
///
/// fn main() {
///     let error = error!("This is a test");
///     assert_eq!(
///         "Voxelar error (Custom): This is a test",
///         format!("{}", error)
///     );
/// }
/// ```
#[macro_export]
macro_rules! error {
    ($($arg:tt),*) => {
        crate::result::VoxelarError::Custom(format!($($arg),*))
    }
}

/// A macro that wraps a formatted string in the error variant `VoxelarError::Custom` and returns
/// this value wrapped in a `Result`.
///
/// This macro's arguments match against any amount of token trees, separated by commas.
/// This data is then passed to the `format` macro, from which the message will be created.
///
/// # Examples
///
/// ```should_panic
/// use voxelar::*;
/// use voxelar::result::*;
///
/// fn main() -> Result<()> {
///     bail!("This should panic.");
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! bail {
    ($($arg:tt),*) => {
        return Err(crate::error!($($arg),*))
    }
}

/// A macro that wraps a formatted string in the error variant `VoxelarError::Custom` and returns
/// this value wrapped in a `Result` only if the specified condition is truthy.
///
/// The first argument of this macro matches against an expression that will be checked for
/// truthfulness.
///
/// The rest of this macro's arguments match against any amount of token trees, separated by
/// commas. This data is then passed to the `format` macro, from which the message will be created.
///
/// # Examples
///
/// ```
/// use voxelar::*;
/// use voxelar::result::*;
///
/// fn main() -> Result<()> {
///     verify!(12 < 24, "12 is apparently more than or equal to 24");
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! verify {
    ($value:expr, $($arg:tt),*) => {
        if !{ $value } {
            return Err(crate::error!($($arg),*));
        }
    }
}

/// This is a trait that can be used to convert any value into voxelar's `Result` type, while
/// attaching some further information in the form of a string.
pub trait Context<T> {
    /// This function converts the value into a value of `Result<T>`.
    fn context(self, ctx: String) -> Result<T>;
}

impl<T> Context<T> for Option<T> {
    fn context(self, ctx: String) -> Result<T> {
        match self {
            Some(value) => Ok(value),
            None => bail!("{}", ctx),
        }
    }
}

impl<T, E: std::error::Error> Context<T> for std::result::Result<T, E> {
    fn context(self, ctx: String) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => bail!("{}: {}", ctx, err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_custom_error() {
        let error = VoxelarError::Custom(format!("Code {}", 5));
        let expected = format!("Voxelar error (Custom): Code 5");
        let actual = format!("{}", error);

        assert_eq!(expected, actual);
    }

    #[derive(Debug)]
    struct Wrapped {
        #[allow(unused)]
        data: u32,
    }

    impl std::fmt::Display for Wrapped {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl std::error::Error for Wrapped {}

    #[test]
    fn format_wrapped_error() {
        let error = VoxelarError::Wrapped(Box::new(Wrapped { data: 12 }));
        let expected = "Voxelar error (Wrapped): Wrapped { data: 12 }";
        let actual = format!("{}", error);

        assert_eq!(expected, actual);
    }

    fn produce_wrapped_error() -> Result<()> {
        Err(VoxelarError::Wrapped(Box::new(Wrapped { data: 1 })))
    }

    #[test]
    fn return_with_wrapped_error() {
        let result = produce_wrapped_error().unwrap_err();
        let expected = "Voxelar error (Wrapped): Wrapped { data: 1 }";
        let actual = format!("{}", result);

        assert_eq!(expected, actual);
    }
}
