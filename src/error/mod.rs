use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)] 
pub enum Error {
    //lexer erorrs
    #[error("UB:Cannot parse str to token" )]
    ParseStrToToken,

    #[error("UB:Cannot parse token to str")]
    ParseTokenToStr, 

    #[error("UB:parse value to str")]
    ParseValueToStr,

    #[error("UnknownSymbol: {0} at pos: {1}")]
    UnknownSymbol(char, usize),

    #[error("Expected bracket")]
    UnmatchedBracket,
    
    #[error("Error while parsing number: {0}")]
    ParsingNumber(String),
}
