use crate::MathType;
use crate::vec_macro::make_vec_type;

make_vec_type! {
    Vec2 {
        size = 2,
        x: 0,
        y: 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vec2_with_members() {
        let vec = Vec2::new(-5, 3);
        assert_eq!(-5, vec.x());
        assert_eq!(3, vec.y());
    }
    
    #[test]
    fn neg_vec2() {
        let vec = Vec2::new(1.0, -5.0);
        let expected = Vec2::new(-1.0, 5.0);

        assert_eq!(expected, -vec);
    }

    #[test]
    fn add_vec2() {
        let first = Vec2::new(4, -3);
        let second = Vec2::new(1, 0);
        let expected = Vec2::new(5, -3);

        assert_eq!(expected, first + second);
    }

    #[test]
    fn subtract_vec2() {
        let first = Vec2::new(0, 10);
        let second = Vec2::new(-12, 4);
        let expected = Vec2::new(12, 6);

        assert_eq!(expected, first - second);
    }

    #[test]
    fn scale_vec2() {
        let first = Vec2::new(12.0, 5.0);
        let expected = Vec2::new(18.0, 7.5);

        assert_eq!(expected, first * 1.5);
    }

    #[test]
    fn multiply_vec2() {
        let first = Vec2::new(4.0, -6.0);
        let second = Vec2::new(-2.0, -2.5);
        let expected = Vec2::new(-8.0, 15.0);

        assert_eq!(expected, first * second);
    }

    #[test]
    fn divide_vec2_by_scalar() {
        let first = Vec2::new(12.0, -8.0);
        let expected = Vec2::new(3.0, -2.0);

        assert_eq!(expected, first / 4.0);
    }
}
