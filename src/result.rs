#[derive(Debug)]
pub enum VoxelarError {
    Wrapped(Box<dyn std::error::Error>),
    Custom(String)
}

impl std::fmt::Display for VoxelarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoxelarError::Wrapped(err) => write!(f, "Voxelar error (Wrapped): {}", err),
            VoxelarError::Custom(msg) => write!(f, "Voxelar error (Custom): {}", msg),
        }
    }
}

impl From<Box<dyn std::error::Error>> for VoxelarError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        Self::Wrapped(value)
    }
}

impl From<String> for VoxelarError {
    fn from(value: String) -> Self {
        Self::Custom(value)
    }
}

impl std::error::Error for VoxelarError {}

pub type Result<T> = std::result::Result<T, VoxelarError>;

macro_rules! error {
    ($($arg:tt),*) => {
        crate::result::VoxelarError::Custom(format!($($arg),*))
    }
}

macro_rules! bail {
    ($($arg:tt),*) => {
        return Err(crate::result::error!($($arg),*))
    }
}

pub(super) use error;
pub(super) use bail;

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
        data: u32
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
