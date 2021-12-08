mod Value;
mod Lexer;
fn main() {
    let mut lexer = Lexer::Lexer::new("!= =");
    if let Err(why) = lexer.lex(){
        println!("{:?}",why);
    }
    println!("{:?}",lexer);
}
