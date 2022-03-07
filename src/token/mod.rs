use crate::error::*;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    //Math operators
    Plus(u8),
    Minus(u8),
    Mul(u8),
    Div(u8),
    Rem(u8),
    Eq(u8),
    NotEq(u8),
    Greater(u8),
    Less(u8),
    EqOrGreater(u8),
    EqOrLess(u8),
    //Boolean operators
    Not(u8),
    Or(u8),
    And(u8),
    //Brackets
    LeftBracket(u8),
    RightBracket(u8),
    //Number
    Value(u8),
}

impl TryInto<String> for TokenKind {
    type Error = Error;
    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Self::Plus(_)  => Ok(String::from("+")),
            Self::Minus(_) => Ok(String::from("-")),
            Self::Mul(_)   => Ok(String::from("*")),
            Self::Div(_)   => Ok(String::from("/")),
            Self::Rem(_)   => Ok(String::from("%")),
            Self::Eq(_)    => Ok(String::from("=")),
            Self::NotEq(_) => Ok(String::from("!=")),
            Self::Not(_)   => Ok(String::from("!")),
            Self::Or(_)    => Ok(String::from("|")),
            Self::And(_)   => Ok(String::from("&")),
            Self::Greater(_) => Ok(String::from(">")),
            Self::Less(_)  => Ok(String::from("<")),
            Self::EqOrGreater(_) => Ok(String::from(">=")),
            Self::EqOrLess(_) => Ok(String::from("<=")),
            Self::LeftBracket(_) => Ok(String::from("(")),
            Self::RightBracket(_) => Ok(String::from(")")),
            Self::Value(_) => Err(Error::ParseValueToStr),
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct Token {
    kind: TokenKind,
    text: String, 
}

impl TryFrom<String> for Token {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Error> {
        match value.as_str() {
            "+" => Ok(Token::new(TokenKind::Plus(0))),
            "-" => Ok(Token::new(TokenKind::Minus(0))),
            "*" => Ok(Token::new(TokenKind::Mul(0))),
            "/" => Ok(Token::new(TokenKind::Div(0))),
            "%" => Ok(Token::new(TokenKind::Rem(0))),
            "=" => Ok(Token::new(TokenKind::Eq(0))),
            ">" => Ok(Token::new(TokenKind::Greater(0))),
            "<" => Ok(Token::new(TokenKind::Less(0))),
            "!" => Ok(Token::new(TokenKind::Not(0))),
            "|" => Ok(Token::new(TokenKind::Or(0))),
            "&" => Ok(Token::new(TokenKind::And(0))),
            "!=" => Ok(Token::new(TokenKind::NotEq(0))),
            ">=" => Ok(Token::new(TokenKind::EqOrGreater(0))),
            "<=" => Ok(Token::new(TokenKind::EqOrLess(0))),
            "(" => Ok(Token::new(TokenKind::LeftBracket(0))),
            ")" => Ok(Token::new(TokenKind::RightBracket(0))),
            _ => Err(Error::ParseStrToToken),
        }
    }
}

impl Token {
    pub fn new(kind:TokenKind) -> Self {
        Self { 
            kind: kind.clone(), 
            text: kind.try_into().unwrap()
        }
    }

    pub fn new_val(v:f64) -> Self {
        Self { 
            kind: TokenKind::Value(0),
            text: v.to_string() 
        }
    }

    pub fn is_value(&self) -> bool {
        matches!(self.kind, TokenKind::Value(_))
    }
}
