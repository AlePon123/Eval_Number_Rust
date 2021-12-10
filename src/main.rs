mod Value;
mod Lexer;
fn main() {
    let mut lexer = Lexer::Lexer::new("-228.1");
    if let Err(why) = lexer.lex(){
        println!("{:?}",why);
    }
    println!("{:?}",lexer);
}
