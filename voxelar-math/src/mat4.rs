use crate::mat_macros::*;
use crate::MathType;

make_mat_type! {
    Mat4x2 {
        size = (4, 2)
    }
}
impl_mat_math!(Mat4x2);

make_mat_type! {
    Mat4x3 {
        size = (4, 3)
    }
}
impl_mat_math!(Mat4x3);

make_mat_type! {
    Mat4 {
        size = (4, 4)
    }
}
impl_mat_math!(Mat4);
