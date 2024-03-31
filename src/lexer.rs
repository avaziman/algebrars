use std::str::FromStr;

use bimap::BiMap;
use rust_decimal::Decimal;

use crate::{MathToken, OperationToken};

#[derive(Clone, Debug, PartialEq)]
pub struct Lexer {
    pub(crate) tokens: Vec<MathToken>,
}

lazy_static::lazy_static! {
    /// This is an example for using doc comment attributes
    pub static ref OPERATOR_MAP: BiMap<char, MathToken> = bimap::BiMap::from_iter(vec![
        ('+' , MathToken::Op(OperationToken::Add)),
        ('-' , MathToken::Op(OperationToken::Subtract)),
        ('/' , MathToken::Op(OperationToken::Divide)),
        ('*' , MathToken::Op(OperationToken::Multiply)),
        ('(' , MathToken::Op(OperationToken::LParent)),
        (')' , MathToken::Op(OperationToken::RParent)),
    ]);
}

impl Lexer {
    pub fn new(str: &str) -> Self {
        let mut chars = str.char_indices().peekable();
        let mut tokens = Vec::new();
        while let Some((i, c)) = chars.next() {
            let token = match c {
                c if c.is_whitespace() => continue,
                c if c.is_ascii_digit() => {
                    let mut str_stop = i + 1;
                    while chars
                        .next_if(|(_, c)| c.is_ascii_digit() || *c == '.')
                        .is_some()
                    {
                        str_stop += 1;
                    }
                    MathToken::Constant(Decimal::from_str(&str[i..str_stop]).unwrap())
                }
                c if c.is_alphabetic() => {
                    let mut str_stop = i + 1;
                    while chars.next_if(|(_, c)| c.is_alphanumeric()).is_some() {
                        str_stop += 1;
                    }
                    MathToken::Variable(str[i..str_stop].to_string())
                }
                _ => match OPERATOR_MAP.get_by_left(&c) {
                    Some(s) => s.clone(),
                    None => panic!("Unhandled char {}", c),
                },
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
                MathToken::Op(OperationToken::Multiply),
                MathToken::Variable("x".to_string())
            ]
        );
    }

    #[test]
    fn lex_parentheses() {
        assert_eq!(
            Lexer::new("2 * (x + 1)").tokens,
            vec![
                MathToken::Constant(dec!(2)),
                MathToken::Op(OperationToken::Multiply),
                MathToken::Op(OperationToken::LParent),
                MathToken::Variable("x".to_string()),
                MathToken::Op(OperationToken::Add),
                MathToken::Constant(dec!(1)),
                MathToken::Op(OperationToken::RParent),
            ]
        );
    }
}
