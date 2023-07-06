use crate::MathType;
use crate::vec_macro::make_vec_type;

make_vec_type! {
    Vec4 {
        size = 4,
        x: 0,
        y: 1,
        z: 2,
        w: 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vec4_with_members() {
        let vec = Vec4::new(0, -7, 2, 3);
        assert_eq!(0, vec.x());
        assert_eq!(-7, vec.y());
        assert_eq!(2, vec.z());
        assert_eq!(3, vec.w());
    }

    #[test]
    fn add_vec4() {
        let first = Vec4::new(1, 2, 9, -3);
        let second = Vec4::new(-1, 3, 0, -4);
        let expected = Vec4::new(0, 5, 9, -7);

        assert_eq!(expected, first + second);
    }

    #[test]
    fn subtract_vec4() {
        let first = Vec4::new(0, 10, 1, 9);
        let second = Vec4::new(-6, -12, 4, -4);
        let expected = Vec4::new(6, 22, -3, 13);

        assert_eq!(expected, first - second);
    }

    #[test]
    fn scale_vec4() {
        let first = Vec4::new(-2.0, 5.0, 12.0, 5.0);
        let expected = Vec4::new(-3.0, 7.5, 18.0, 7.5);

        assert_eq!(expected, first * 1.5);
    }

    #[test]
    fn multiply_vec4() {
        let first = Vec4::new(4.0, 1.0, -6.0, 0.0);
        let second = Vec4::new(-2.0, 0.5, -2.5, 125.0);
        let expected = Vec4::new(-8.0, 0.5, 15.0, 0.0);

        assert_eq!(expected, first * second);
    }

    #[test]
    fn divide_vec4_by_scalar() {
        let first = Vec4::new(12.0, -8.0, 0.5, 9.0);
        let expected = Vec4::new(3.0, -2.0, 0.125, 2.25);

        assert_eq!(expected, first / 4.0);
    }
}
