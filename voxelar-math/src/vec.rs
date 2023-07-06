macro_rules! make_vec_type {
    ($name:ident {
        size = $size:expr,
        $(
            $member:ident: $idx:expr
        ),*
    }) => {
        #[repr(C)]
        pub struct $name<T: MathType> {
            values: [T; $size]
        }

        impl<T: MathType> $name<T> {
            pub fn new($($member: T),*) -> Self {
                Self {
                    values: [$($member),*]
                }
            }

            $(
            pub fn $member(&self) -> T {
                self.values[$idx]
            }
            )*

            pub fn as_ptr(&self) -> *const T {
                self.values.as_ptr()
            }
        }

        impl<T: MathType + std::ops::Add<Output = T>> std::ops::Add for $name<T> {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self::new(
                    $( self.$member() + other.$member() ),*
                )
            }
        }

        impl<T: MathType + std::ops::Sub<Output = T>> std::ops::Sub for $name<T> {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self::new(
                    $( self.$member() - other.$member() ),*
                )
            }
        }

        impl<T: MathType + std::ops::Mul<Output = T>> std::ops::Mul<T> for $name<T> {
            type Output = Self;

            fn mul(self, other: T) -> Self {
                Self::new(
                    $( self.$member() * other ),*
                )
            }
        }

        impl<T: MathType + std::ops::Mul<Output = T>> std::ops::Mul for $name<T> {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                Self::new(
                    $( self.$member() * other.$member() ),*
                )
            }
        }

        impl<T: MathType + std::ops::Div<Output = T>> std::ops::Div<T> for $name<T> {
            type Output = Self;

            fn div(self, other: T) -> Self {
                Self::new(
                    $( self.$member() / other ),*
                )
            }
        }
    }
}

pub(super) use make_vec_type;
