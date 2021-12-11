use std::iter::Peekable;
mod token;
use token::*;

#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter,
}

#[derive(Debug,Default)]
pub struct Lexer<'a> {
    start: usize,
    end: usize,
    curr: usize,
    input: &'a str,
    tokens:Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input:&'a str) -> Self {
        Self {
            input,
            ..std::default::Default::default()
        }
    } 

    pub fn lex(&mut self) -> Result<(), LexerError> {
        let mut iter = self.input.chars().peekable();
        while let Some(&c) = iter.peek() {
            match c {
                '0'..='9' => {
                    let n = get_value(&mut iter);
                    let tok = Token::new_val(n);
                    self.tokens.push(tok);
                }
                '-' => {
                    match get_value_with_minus(&mut iter) {
                        Some(v) => self.tokens.push(Token::new_val(v)),
                        None => self.tokens.push(Token::new(TokenKind::Minus(0))),
                    }
                }
                '+' => {
                    self.tokens.push(Token::new(TokenKind::Plus(0)));
                    iter.next();
                }
                '*' => {
                    self.tokens.push(Token::new(TokenKind::Mul(0)));
                    iter.next();
                }
                '/' => {
                    self.tokens.push(Token::new(TokenKind::Div(0)));
                    iter.next();
                }
                '!' => {
                    iter.next();
                    if next_is_equal(&mut iter) {
                        self.tokens.push(Token::new(TokenKind::Not(0)));
                    } else {
                        return Err(LexerError::UnexpectedCharacter)
                    }
                }
                '=' => {
                    self.tokens.push(Token::new(TokenKind::Eq(0)));
                    iter.next();
                }
                _ if is_whitespace(c) => { iter.next(); }
                _ => todo!()
            }
        }
        Ok(())
    }
}

fn get_value<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> f64 {
    let number = iter.take_while(|c| matches!(c, '0'..='9' | '.')).collect::<String>();
    number.parse::<f64>().unwrap()
}

fn get_value_with_minus<T:Iterator<Item = char>>(iter:&mut Peekable<T>) -> Option<f64> {
    iter.next();
    let number = iter.take_while(|c| matches!(c, '0'..='9' | '.')).collect::<String>();
    if number.is_empty() { return None } 
    Some(number.parse::<f64>().unwrap() * -1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lexer(input: &str, tokens:&[Token]) {
        let mut lexer = Lexer::new(input);
        if let Err(e) = lexer.lex() {
            eprintln!("{:?}",e);
        }
        lexer.tokens.iter().zip(tokens.iter())
            .for_each(|(self_tok , test_tok)| assert_eq!(self_tok,test_tok) );
    }

    #[test]
    fn operators_test() {
        let tokens = vec![
            Token::new(TokenKind::Minus(0)),
            Token::new(TokenKind::Plus(0)),
            Token::new(TokenKind::Mul(0)),
            Token::new(TokenKind::Div(0)),
            Token::new(TokenKind::Not(0)),
            Token::new(TokenKind::Eq(0)),
        ];
        test_lexer("- + * / != =", &tokens)
    } 

    #[test]
    fn test_numbers() {
        let tokens = vec![
            Token::new_val(-69.5),
            Token::new(TokenKind::Plus(0)),
            Token::new_val(-420.3123),
            Token::new(TokenKind::Minus(0)),
            Token::new_val(228.1),
            Token::new(TokenKind::Mul(0)),
            Token::new_val(322.0),
        ];
        test_lexer("-69.5 + -420.3123 - 228.1 * 322",&tokens);
    }
}
