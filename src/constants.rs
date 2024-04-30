use std::collections::HashMap;

use rust_decimal::Decimal;


lazy_static::lazy_static! {
    pub static ref CONSTANTS_MAP: HashMap<&'static str, Decimal> = HashMap::from([
        ("pi", Decimal::PI),
        ("π", Decimal::PI),
        ("e", Decimal::E),
    ]);
}