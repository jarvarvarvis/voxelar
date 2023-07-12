use crate::vec_macro::make_vec_type;
use crate::MathType;

make_vec_type! {
    Quaternion {
        size = 4,
        x: 0,
        y: 1,
        z: 2,
        w: 3
    }
}
