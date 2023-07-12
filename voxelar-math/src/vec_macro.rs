macro_rules! make_vec_type {
    ($name:ident {
        size = $size:expr,
        $(
            $member:ident: $idx:expr
        ),*
    }

    impl {
        $(
            $further_impl_stmts:stmt
        ),*
    }) => {
        #[repr(C)]
        #[derive(PartialEq, Debug, Clone, Copy)]
        pub struct $name<T: MathType> {
            values: [T; $size]
        }

        impl<T: MathType> $name<T> {
            pub fn new($($member: T),*) -> Self {
                Self {
                    values: [$($member),*]
                }
            }

            pub fn into_values(self) -> [T; $size] {
                self.values
            }

            $(
            pub fn $member(&self) -> T {
                self.values[$idx]
            }
            )*

            $(
                $further_impl_stmts
            )*
        }

        impl<T: MathType + std::ops::Neg<Output = T>> std::ops::Neg for $name<T> {
            type Output = Self;

            fn neg(self) -> Self {
                Self::new(
                    $( -self.$member() ),*
                )
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
    };

    ($name:ident {
        size = $size:expr,
        $(
            $member:ident: $idx:expr
        ),*
    }) => {
        crate::vec_macro::make_vec_type! {
            $name {
                size = $size,
                $(
                    $member: $idx
                ),*
            }

            impl {
            }
        }
    }
}

pub(super) use make_vec_type;
