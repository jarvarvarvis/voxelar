use crate::MathType;
use crate::vec_macro::make_vec_type;

make_vec_type! {
    Vec3 {
        size = 3,
        x: 0,
        y: 1,
        z: 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vec3_with_members() {
        let vec = Vec3::new(-7, 1, 3);
        assert_eq!(-7, vec.x());
        assert_eq!(1, vec.y());
        assert_eq!(3, vec.z());
    }
    
    #[test]
    fn neg_vec3() {
        let vec = Vec3::new(-1.0, 0.125, 5.0);
        let expected = Vec3::new(1.0, -0.125, -5.0);

        assert_eq!(expected, -vec);
    }

    #[test]
    fn add_vec3() {
        let first = Vec3::new(2, 9, -3);
        let second = Vec3::new(3, 0, -4);
        let expected = Vec3::new(5, 9, -7);

        assert_eq!(expected, first + second);
    }

    #[test]
    fn subtract_vec3() {
        let first = Vec3::new(0, 10, 1);
        let second = Vec3::new(-6, -12, 4);
        let expected = Vec3::new(6, 22, -3);

        assert_eq!(expected, first - second);
    }

    #[test]
    fn scale_vec3() {
        let first = Vec3::new(-2.0, 12.0, 5.0);
        let expected = Vec3::new(-3.0, 18.0, 7.5);

        assert_eq!(expected, first * 1.5);
    }

    #[test]
    fn multiply_vec3() {
        let first = Vec3::new(4.0, 1.0, -6.0);
        let second = Vec3::new(-2.0, 0.5, -2.5);
        let expected = Vec3::new(-8.0, 0.5, 15.0);

        assert_eq!(expected, first * second);
    }

    #[test]
    fn divide_vec3_by_scalar() {
        let first = Vec3::new(12.0, -8.0, 0.5);
        let expected = Vec3::new(3.0, -2.0, 0.125);

        assert_eq!(expected, first / 4.0);
    }
}
