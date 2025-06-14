mod tokenizer;
mod lexer;
mod parser;
mod cursor;

use crate::parser::Node;

const CODE: &str = include_str!("../maaray-examples/fibonacci.mry");

fn parse_to_ast(code: &str) -> Node {
	let tokenizer = tokenizer::Tokenizer::new(code);
	let lexer = lexer::Lexer::new(tokenizer);
	let tokens: Vec<_> = lexer.collect();

	for i in &tokens {
	    if let Err(e) = i {
	        eprintln!("Tokenizing error at {}:{}", e.line(), e.column());
	    }
	}

	let tokens = tokens.into_iter().map(|a| a.unwrap()).collect();

	println!("{:?}", tokens);

	let mut parser = parser::Parser::new(tokens);
	let ast = parser.parse();

	ast
}

fn main() {
	let filename = match std::env::args().skip(1).next() {
		Some(x) => x,
		None => {
			eprintln!("Usage: {} code.mry", std::env::args().nth(0).unwrap());
			std::process::exit(1);
		}
	};

	let code = std::fs::read_to_string(filename).expect("File error");

	println!("Code: \n{}", &code);
	println!("----------------------");

	let ast = parse_to_ast(&code);

    println!("Tokens: {:#?}", ast);
}
