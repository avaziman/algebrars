use std::{collections::HashMap, rc::Rc, str::FromStr};

use rust_decimal::Decimal;

use crate::{MathToken, OperationToken};

#[derive(Clone, Debug, PartialEq)]
pub struct Lexer {
    pub(crate) tokens: Vec<MathToken>,
}

impl Lexer {
    pub fn new(str: &str) -> Self {
        let mut chars = str.char_indices().peekable();
        let mut tokens = Vec::new();
        let mut variables: HashMap<&str, Rc<String>> = HashMap::new();

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
                    MathToken::constant(Decimal::from_str(&str[i..str_stop]).unwrap())
                }
                c if c.is_alphabetic() => {
                    let mut str_stop = i + 1;
                    while chars.next_if(|(_, c)| c.is_alphanumeric()).is_some() {
                        str_stop += 1;
                    }

                    let var = &str[i..str_stop];
                    // avoid allocating same variable string twice
                    MathToken::variable(match variables.get(var) {
                        Some(v) => v.clone(),
                        None => {
                            let rc = Rc::new(var.to_string());
                            let rc2 = rc.clone();
                            variables.insert(var, rc);
                            rc2
                        }
                    })
                }
                _ => match OperationToken::from_char(c) {
                    Some(s) => MathToken::operator(s.clone()),
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
                MathToken::constant(dec!(2)),
                MathToken::operator(OperationToken::Multiply),
                MathToken::variable("x".to_string())
            ]
        );
    }

    #[test]
    fn lex_parentheses() {
        assert_eq!(
            Lexer::new("2 * (x + 1)").tokens,
            vec![
                MathToken::constant(dec!(2)),
                MathToken::operator(OperationToken::Multiply),
                MathToken::operator(OperationToken::LParent),
                MathToken::variable("x".to_string().into()),
                MathToken::operator(OperationToken::Add),
                MathToken::constant(dec!(1)),
                MathToken::operator(OperationToken::RParent),
            ]
        );
    }
}
