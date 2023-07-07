#[derive(Debug)]
pub enum VoxelarMathError {
    InvalidFlatSize2D {
        expected_dim: (usize, usize),
        got: usize,
    },
}

impl std::fmt::Display for VoxelarMathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoxelarMathError::InvalidFlatSize2D { expected_dim, got } => {
                write!(f, "Value has invalid dimensions! Expected: {:?} (= {}), got: {:?}", 
                       expected_dim, expected_dim.0 * expected_dim.1, got)
            },
        }
    }
}

impl std::error::Error for VoxelarMathError {}

pub type Result<T> = std::result::Result<T, VoxelarMathError>;
