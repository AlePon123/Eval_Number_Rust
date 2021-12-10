#[derive(Debug,PartialEq,Clone)]
pub enum TokenKind {
    Plus(u8),
    Minus(u8),
    Mul(u8),
    Div(u8), 
    Eq(u8),
    Not(u8),
    Value(u8),
}

impl TryInto<String> for TokenKind {
    type Error = ExpectedStrToken;
    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Self::Plus(_)  => Ok(String::from("+")),
            Self::Minus(_) => Ok(String::from("-")),
            Self::Mul(_)   => Ok(String::from("*")),
            Self::Div(_)   => Ok(String::from("/")),
            Self::Eq(_)    => Ok(String::from("=")),
            Self::Not(_)   => Ok(String::from("!=")),
            _ => Err(ExpectedStrToken),
        }
    }
}

#[derive(Debug)]
pub struct ExpectedStrToken;

#[derive(Debug,PartialEq)]
pub struct Token {
    kind: TokenKind,
    text: String, 
}

impl Token {
    pub fn new(kind:TokenKind,) -> Self {
        Self { kind: kind.clone(), text: kind.try_into().unwrap()}
    }
    pub fn new_val(v:f64) -> Self {
        Self {kind: TokenKind::Value(0) , text: v.to_string()}
    } 
    pub fn is_value(&self) -> bool {
        matches!(self.kind, TokenKind::Value(_))
    }
}

pub fn is_whitespace(c: char) -> bool {
    c == ' '
}

pub fn next_is_equal<T: Iterator<Item = char>>(iter:&mut std::iter::Peekable<T>) -> bool {
    match iter.next() {
        Some(c) => {
            iter.next();
            c == '='
        }
        None => false
    }
}
