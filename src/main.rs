pub mod lexer;
pub mod token;
pub mod error;

fn main() {
    let input = String::from("228.1");
    let mut lexer = lexer::Lexer::new(&input);
    if let Err(why) = lexer.lex(){
        println!("Error: {}",why);
    }
    println!("Tokens: {:?}\n",lexer.tokens,);
}
