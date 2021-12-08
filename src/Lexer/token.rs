use std::str::FromStr;
#[derive(Debug,PartialEq)]
pub enum TokenKind {
    Plus { 
        priority:u8
    },
    Minus {
        priority:u8
    },
    Mul { 
        priority:u8
    },
    Div {
        priority:u8
    },
    // bolloean ops
    Eq {
        priority:u8
    },
    Not {
        priority:u8
    },

    Value {
        val:f64,
        priority:u8
    },
}
#[derive(Debug)]
pub struct Expected_str_token;

impl FromStr for TokenKind {
    type Err = Expected_str_token;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "plus"  => Ok(Self::Plus    {priority:0}),
            "minus" => Ok(Self::Minus   {priority:0}),
            "mul"   => Ok(Self::Mul     {priority:0}),
            "div"   => Ok(Self::Div     {priority:0}),
            "eq"    => Ok(Self::Eq      {priority:0}),
            "not"   => Ok(Self::Not     {priority:0}),
            _ => Err(Expected_str_token),
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct Token {
    kind: TokenKind,
}

impl Token {
    pub fn new(s: &str) -> Self {
        Self { kind: TokenKind::from_str(s).unwrap() }
    }
    pub fn new_val(v:f64) -> Self {
        Self { kind: TokenKind::Value {val:v, priority:0}}
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
