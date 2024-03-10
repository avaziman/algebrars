use number::Number;

pub mod number;

enum Operations {
    Pow,
    Root,
    Minus,
    Plus,
    Divide,
    Multiply,
}


enum MathToken {
    Constant(Number),
    Variable(String),
}


#[cfg(test)]
mod tests {
    use crate::number::{FIXED_DECIMAL_POINTS, FIXED_DECIMAL_POINTS_MUL};

    use super::*;

    #[test]
    fn divide_accuracy() {
        let result = Number::int(1) / Number::int(3);
        assert_eq!(result, (Number::int_decimal(33333333, FIXED_DECIMAL_POINTS as u8), true));

        let result = Number::int(1) / Number::int(2);
        assert_eq!(result, (Number::int_decimal(5, 1), false));
    }
}
