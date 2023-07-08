macro_rules! make_mat_type {
    ($name:ident {
        size = ($columns:expr, $rows:expr)
    }) => {
        #[derive(Debug, PartialEq, Clone)]
        pub struct $name<T: MathType> {
            values: Vec<T>
        }

        impl<T: MathType> $name<T> {
            pub const COLUMNS: usize = $columns;
            pub const ROWS: usize = $rows;

            pub fn new(matrix: [[T; $columns]; $rows]) -> Self {
                let mut values = Vec::with_capacity(Self::ROWS * Self::COLUMNS);

                for column in 0..Self::COLUMNS {
                    for row in 0..Self::ROWS {
                        values.push(matrix[row][column]);
                    }
                }

                Self { values }
            }

            pub fn rows(&self) -> usize {
                Self::ROWS
            }

            pub fn columns(&self) -> usize {
                Self::COLUMNS
            }

            pub fn get(&self, row: usize, column: usize) -> T {
                self.values[column * Self::ROWS + row]
            }

            pub fn get_ref(&self, row: usize, column: usize) -> &T {
                &self.values[column * Self::ROWS + row]
            }

            pub fn get_mut(&mut self, row: usize, column: usize) -> &mut T {
                &mut self.values[column * Self::ROWS + row]
            }

            pub fn as_ptr(&self) -> *const T {
                self.values.as_ptr()
            }
        }

        impl<T: MathType> std::ops::Index<(usize, usize)> for $name<T> {
            type Output = T;

            fn index(&self, index: (usize, usize)) -> &Self::Output {
                self.get_ref(index.0, index.1) 
            }
        }
    };
}

pub(super) use make_mat_type;

macro_rules! impl_mat_math {
    ($name:ident) => {
        impl<T: MathType + std::ops::Add<Output = T>> std::ops::Add for $name<T> {
            type Output = Self;

            fn add(mut self, rhs: Self) -> Self::Output {
                for column in 0..Self::COLUMNS {
                    for row in 0..Self::ROWS {
                        *self.get_mut(row, column) = self.get(row, column) + rhs.get(row, column);
                    }
                }
                self
            }
        }

        impl<T: MathType + std::ops::Sub<Output = T>> std::ops::Sub for $name<T> {
            type Output = Self;

            fn sub(mut self, rhs: Self) -> Self::Output {
                for column in 0..Self::COLUMNS {
                    for row in 0..Self::ROWS {
                        *self.get_mut(row, column) = self.get(row, column) - rhs.get(row, column);
                    }
                }
                self
            }
        }

        impl<T: MathType + std::ops::Mul<Output = T>> std::ops::Mul for $name<T> {
            type Output = Self;

            fn mul(mut self, rhs: Self) -> Self::Output {
                for column in 0..Self::COLUMNS {
                    for row in 0..Self::ROWS {
                        *self.get_mut(row, column) = self.get(row, column) * rhs.get(row, column);
                    }
                }
                self
            }
        }

        impl<T: MathType + std::ops::Mul<Output = T>> std::ops::Mul<T> for $name<T> {
            type Output = Self;

            fn mul(mut self, rhs: T) -> Self::Output {
                for column in 0..Self::COLUMNS {
                    for row in 0..Self::ROWS {
                        *self.get_mut(row, column) = self.get(row, column) * rhs;
                    }
                }
                self
            }
        }

        impl<T: MathType + std::ops::Div<Output = T>> std::ops::Div<T> for $name<T> {
            type Output = Self;

            fn div(mut self, rhs: T) -> Self::Output {
                for column in 0..Self::COLUMNS {
                    for row in 0..Self::ROWS {
                        *self.get_mut(row, column) = self.get(row, column) / rhs;
                    }
                }
                self
            }
        }
    };
}

pub(super) use impl_mat_math;
