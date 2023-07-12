pub trait Identity {
    fn identity() -> Self;
}

macro_rules! impl_identity_for_type {
    ($type:ty, $identity_value:expr) => {
        impl Identity for $type {
            fn identity() -> Self {
                $identity_value
            }
        }
    };
}

impl_identity_for_type!(usize, 1);
impl_identity_for_type!(u8, 1);
impl_identity_for_type!(u16, 1);
impl_identity_for_type!(u32, 1);
impl_identity_for_type!(u64, 1);
impl_identity_for_type!(u128, 1);

impl_identity_for_type!(isize, 1);
impl_identity_for_type!(i8, 1);
impl_identity_for_type!(i16, 1);
impl_identity_for_type!(i32, 1);
impl_identity_for_type!(i64, 1);
impl_identity_for_type!(i128, 1);

impl_identity_for_type!(f32, 1.0);
impl_identity_for_type!(f64, 1.0);
