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
                    iter.next(); 
                    let n = get_value(c, &mut iter);
                    let tok = Token::new_val(n);
                    self.tokens.push(tok);
                },
                '-' => {
                    let n = get_value_with_minus(&mut iter);
                    match n {
                        Some(v) => self.tokens.push(Token::new_val(v)),
                        None => self.tokens.push(Token::new("minus")),
                    }
                }
                '+' => {
                    self.tokens.push(Token::new("plus"));
                    iter.next();
                },
                '*' => {
                    self.tokens.push(Token::new("mul"));
                    iter.next();
                },
                '/' => {
                    self.tokens.push(Token::new("div"));
                    iter.next();
                },
                '!' => {
                    iter.next();
                    if next_is_equal(&mut iter) {
                        self.tokens.push(Token::new("not"));
                    } else {
                        return Err(LexerError::UnexpectedCharacter)
                    }
                }
                '=' => {
                    self.tokens.push(Token::new("eq"));
                    iter.next();
                }

                _ if is_whitespace(c) => { iter.next(); }
                _ => todo!()
            }
        }
        Ok(())
    }
}

//TODO Сделать нормальный лексер
//https://github.com/tjdevries/vim9jit/blob/master/src/lexer/mod.rs#L250
fn get_value<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> f64 {
    let mut number = c.to_string().parse::<u64>().expect("");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<u64>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number.to_string().parse::<f64>().unwrap()
}

fn get_value_with_minus<T:Iterator<Item = char>>(iter:&mut Peekable<T>) -> Option<f64> {
    let mut buf = String::new();
    iter.next();
    buf.push('-');
    let number = iter.next().unwrap().to_string().parse::<u128>();
    if let Err(_) = number {
        return None
    } else {
        let mut number = number.unwrap();
        while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<u64>()) {
           number = number * 10 + digit as u128;
           iter.next();
        }
        buf.push_str(&number.to_string());
    }
    Some(buf.parse::<f64>().unwrap())
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
            .for_each(|(self_tok , test_tok)| {
                    assert_eq!(self_tok,test_tok);
                }
            );
    }
    
    fn fill_slice(s:&str) -> Vec::<Token> {
        let mut slice = Vec::<Token>::new();
        let mut tokens = s.split_whitespace();
        while let Some(s) = tokens.next() {
            slice.push(Token::new(s));
        }
        slice
    }

    #[test]
    fn operators_test() {
        let tokens = "minus plus mul div not eq";
        let slice = fill_slice(tokens);
        test_lexer("- + * / != =", &slice)
    } 

    #[test]
    fn test_numbers() {
        let tokens = vec![
            Token::new_val(69.0),
            Token::new("plus"),
            Token::new_val(420.0)
        ];
        test_lexer("69.0 + 420",&tokens);
    }
}
