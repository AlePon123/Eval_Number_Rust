use itertools::Itertools; 
use std::iter::Peekable;
use std::str::Chars;
use crate::token::*;
use crate::error::*;

#[derive(Debug, Clone)] 
pub struct LexIter<'a> {
    chars:Peekable<Chars<'a>>,
}

fn validate_value(number:String) -> Result<f64, Error> {
    let mut counter_dot = 0usize;
    number.chars().for_each(|c| {
        match c {
            '.' => counter_dot += 1,
            _ => {}
        }
    });
    if counter_dot > 1 { return Err(Error::ParsingNumber(number)) };
    Ok(number.parse::<f64>().unwrap())
}

impl<'a> LexIter<'a> {
    fn new(input:&'a String) -> LexIter {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn get_value(&mut self) -> Result<f64, Error> {
        let number = self.take_while_ref(|c| matches!(c, '0'..='9' | '.')).collect::<String>();
        validate_value(number)
    }

    fn get_negative_value(&mut self) -> Result<Option<f64>, Error> {
        self.next();
        let number = self.take_while_ref(|c| matches!(c, '0'..='9' | '.' )).collect::<String>();
        if number.is_empty() { return Ok(None) }
        match validate_value(number) {
            Ok(v) => Ok(Some(v * -1.0)),
            Err(e) => Err(e)
        }
    }

    fn next_is_eq(&mut self) -> bool {
        match self.chars.peek() {
            None => false,
            Some(c) => *c == '='
        }
    }
}

impl Iterator for LexIter<'_> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        self.chars.next()
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    pub iter: LexIter<'a>,
    pub tokens:Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input:&'a String) -> Lexer {
        Self {
            iter: LexIter::new(input),
            tokens: vec![],
        }
    }

    pub fn lex(&mut self) -> Result<(), Error> {
        let iter = &mut self.iter;
        while let Some(&c) = iter.chars.peek() {
            let s = c.to_string();
            match c {
                '0'..='9' => {
                    let n = iter.get_value();
                    match n {
                        Ok(v) => self.tokens.push(Token::new_val(v)),
                        Err(e) => return Err(e),
                    }
                }
                '-' | '+' | '*' | '/' | '%' |'|' | '=' |
                '&' | '(' | ')' => {
                    if c == '-' {
                        match iter.get_negative_value() {
                            Ok(v) => { 
                                match v {
                                    None => self.tokens.push(Token::try_from(s).unwrap()),
                                    Some(n) => self.tokens.push(Token::new_val(n)),
                                }
                            }
                            Err(e)=> return Err(e),
                        }
                    } else {
                        self.tokens.push(Token::try_from(s).unwrap());
                        iter.next();
                    }
                }
                '!' | '>' | '<' => {
                    //if next is equal do double iter.next() like:
                    //[...'>', '=' ,'228'...]
                    //    ---       -----
                    //  cur pos    next pos
                    iter.next();
                    if iter.next_is_eq() {
                        let mut op = String::from(s);
                        op.push('=');
                        self.tokens.push(Token::try_from(op).unwrap());
                        iter.next();
                    } else {
                        self.tokens.push(Token::try_from(s).unwrap());
                    }
                }
                ' '  => { iter.next(); }
                _ => {
                    let pos = iter.position(|r| r == c).unwrap();
                    return Err(Error::UnknownSymbol(c,pos))
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lexer(input: &str, tokens:&[Token]) {
        let s = input.to_string();
        let mut lexer = Lexer::new(&s);
        if let Err(e) = lexer.lex() {
            eprintln!("{:?}",e);
        }
        lexer.tokens.iter().zip(tokens.iter())
            .for_each(|(self_tok , test_tok)| assert_eq!(self_tok,test_tok) );
    }
    #[test]
    fn test_equals() {
        let raw_token = vec![
            Token::new(TokenKind::EqOrLess(0)),
            Token::new(TokenKind::EqOrGreater(0)),
            Token::new(TokenKind::Less(0)),
            Token::new(TokenKind::Greater(0)),
            Token::new(TokenKind::NotEq(0)),
            Token::new(TokenKind::Eq(0)),
        ];
        test_lexer("<= >= < > != =", &raw_token);
    }

    #[test]
    fn test_numbers() {
        let raw_token = vec![
            Token::new_val(1.0),
            Token::new(TokenKind::Plus(0)),
            Token::new_val(-69.5),
            Token::new(TokenKind::Plus(0)),
            Token::new_val(-420.3123),
            Token::new(TokenKind::Minus(0)),
            Token::new_val(228.1),
            Token::new(TokenKind::Mul(0)),
            Token::new_val(322.0),
            Token::new(TokenKind::Rem(0)),
            Token::new_val(-16.9),
            Token::new_val(229.0),
            Token::new(TokenKind::EqOrGreater(0)),
            Token::new_val(228.0),
            Token::new_val(28.0),
            Token::new(TokenKind::Greater(0)),
            Token::new_val(229.0)
        ];
        test_lexer("1 + -69.5 + -420.3123 - 228.1 * 322 % -16.9 229>=228 28>229",&raw_token);
    }

    #[test]
    fn single_ops() {
        let tokens = vec![
            Token::new(TokenKind::Not(0)),
            Token::new(TokenKind::Greater(0)),
            Token::new(TokenKind::Less(0)),
        ];
        test_lexer("! > <", &tokens);
    }

    #[test]
    fn binary_ops() {
        let tokens = vec![
            Token::new(TokenKind::LeftBracket(0)),
            Token::new(TokenKind::LeftBracket(0)),
            Token::new_val(2.0),
            Token::new(TokenKind::Plus(0)),
            Token::new_val(2.0),
            Token::new(TokenKind::RightBracket(0)),
            Token::new(TokenKind::Eq(0)),
            Token::new_val(2.0),
            Token::new(TokenKind::And(0)),
            Token::new(TokenKind::LeftBracket(0)),
            Token::new_val(2.0),
            Token::new(TokenKind::Mul(0)),
            Token::new_val(2.0),
            Token::new(TokenKind::RightBracket(0)),
            Token::new(TokenKind::Eq(0)),
            Token::new_val(2.0),
            Token::new(TokenKind::RightBracket(0)),
        ];
        test_lexer("((2 + 2) = 2 & (2 * 2) = 2)", &tokens);
    }

    #[test]
    fn bracket_test() {
        let tokens = vec![
            Token::new(TokenKind::LeftBracket(0)),
            Token::new(TokenKind::RightBracket(0)),
        ];
        test_lexer("()", &tokens);
    }
}
