// fixed point arithmatic
pub const FIXED_DECIMAL_POINTS : u32 = 8;
pub const FIXED_DECIMAL_POINTS_MUL : i64 = 10i64.pow(FIXED_DECIMAL_POINTS);

#[derive(PartialEq, Debug)]
pub struct Number(i64);

impl Number {
    pub fn int(i: i64) -> Number{
        Number(i * FIXED_DECIMAL_POINTS_MUL)
    }

    pub fn int_decimal(i: i64, decimal: u8) -> Number{
        Number(i * FIXED_DECIMAL_POINTS_MUL / 10i64.pow(decimal as u32))
    }
}

impl std::ops::Div<Number> for Number {
    // lost precision boolean
    type Output = (Number, bool);

    fn div(self, rhs: Number) -> Self::Output {
        let mul = self.0 * FIXED_DECIMAL_POINTS_MUL;
        let res = mul / rhs.0;
        
        (Number(res), res * rhs.0 != mul)
    }
}
