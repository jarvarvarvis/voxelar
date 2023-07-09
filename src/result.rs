use std::ffi::NulError;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum VoxelarError {
    Wrapped(Box<dyn std::error::Error>),
    Custom(String),
    NulError(NulError),
    IOError(std::io::Error),
    Utf8Error(Utf8Error),
    VkError(ash::vk::Result),
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
error_impl_from!(ash::vk::Result, Self::VkError);
error_impl_from!(ash::LoadingError, Self::VkLoadingError);

pub type Result<T> = std::result::Result<T, VoxelarError>;

#[macro_export]
macro_rules! error {
    ($($arg:tt),*) => {
        crate::result::VoxelarError::Custom(format!($($arg),*))
    }
}

#[macro_export]
macro_rules! bail {
    ($($arg:tt),*) => {
        return Err(crate::error!($($arg),*))
    }
}

#[macro_export]
macro_rules! verify {
    ($value:expr, $($arg:tt),*) => {
        if !{ $value } {
            return Err(crate::error!($($arg),*));
        }
    }
}

pub trait Context<T> {
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
