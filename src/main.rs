mod tokenizer;
mod lexer;
mod parser;

const CODE: &str = include_str!("../maaray-examples/fibonacci.mry");

fn main() {
    let tokenizer = tokenizer::Tokenizer::new(CODE);
    // let tokens = tokenizer.collect::<Vec<_>>();

    let lexer = lexer::Lexer::new(tokenizer);
    // let tokens = lexer.collect::<Vec<_>>();

    let parser = parser::Parser::new(lexer);

    println!("Tokens: {:#?}", parser.parse());
}
