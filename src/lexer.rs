use std::str::FromStr;

use rust_decimal::Decimal;

use crate::{MathToken, Operation};

#[derive(Debug, PartialEq)]
pub struct Lexer {
    pub(crate) tokens: Vec<MathToken>,
}

impl Lexer {
    pub fn new(str: &str) -> Self {
        let mut chars = str.chars().enumerate();
        let mut tokens = Vec::new();
        while let Some((i, c)) = chars.next() {
            let token = match c {
                c if c.is_whitespace() => continue,
                c if c.is_numeric() => {
                    let mut str_stop = i + 1;
                    while let Some((_, c)) = chars.next() {
                        if c.is_ascii_digit() || c == '.' {
                            str_stop += 1;
                        } else {
                            break;
                        }
                    }
                    MathToken::Constant(Decimal::from_str(&str[i..str_stop]).unwrap())
                }
                c if c.is_alphabetic() => {
                    let mut str_stop = i + 1;
                    while let Some((_, c)) = chars.next() {
                        if c.is_alphanumeric() {
                            str_stop += 1;
                        } else {
                            break;
                        }
                    }
                    MathToken::Variable(str[i..str_stop].to_string())
                }
                '+' => MathToken::Op(Operation::Plus),
                '-' => MathToken::Op(Operation::Minus),
                '/' => MathToken::Op(Operation::Divide),
                '*' => MathToken::Op(Operation::Multiply),
                _ => panic!("Unhandled char {}", c),
            };
            tokens.push(token);
        }

        Self { tokens }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn lex() {
        assert_eq!(
            Lexer::new("2 * x").tokens,
            vec![
                MathToken::Constant(dec!(2)),
                MathToken::Op(Operation::Multiply),
                MathToken::Variable("x".to_string())
            ]
        );
    }
}
