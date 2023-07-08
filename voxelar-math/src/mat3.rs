use crate::mat_macros::*;
use crate::MathType;

make_mat_type! {
    Mat3x2 {
        size = (3, 2)
    }
}
impl_mat_math!(Mat3x2);

make_mat_type! {
    Mat3 {
        size = (3, 3)
    }
}
impl_mat_math!(Mat3);

make_mat_type! {
    Mat3x4 {
        size = (3, 4)
    }
}
impl_mat_math!(Mat3x4);
