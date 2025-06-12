mod tokenizer;
mod lexer;
mod parser;
mod cursor;

const CODE: &str = include_str!("../maaray-examples/fibonacci.mry");

fn main() {
	println!("Code: \n{CODE}");

	println!("----------------------");

    let tokenizer = tokenizer::Tokenizer::new(CODE);
    let lexer = lexer::Lexer::new(tokenizer);
    let tokens: Vec<_> = lexer.collect();

    for i in &tokens {
        if let Err(e) = i {
            eprintln!("Tokenizing error at {}:{}", e.line(), e.column());
        }
    }

    let tokens = tokens.into_iter().map(|a| a.unwrap()).collect();

    let mut parser = parser::Parser::new(tokens);

    println!("Tokens: {:#?}", parser.parse());
}
