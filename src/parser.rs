use crate::{lexer::Lexem, tokenizer::TResult};

#[derive(Debug, Clone)]
pub enum Node {
    Ident(String),
    Number(f64),
    String(String),
    Assignment {
        name: String,
        value: Box<Node>
    },
    Function {
        name: String,
        argument_names: Vec<Box<Node>>,
        code: Vec<Node>
    },
    Return(Box<Node>),
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    BinOr(Box<Node>, Box<Node>),
    BinAnd(Box<Node>, Box<Node>),
    Not(Box<Node>),
    Or(Box<Node>, Box<Node>),
    And(Box<Node>, Box<Node>),
    Equals(Box<Node>, Box<Node>),
    NotEquals(Box<Node>, Box<Node>),
    Call {
        callee: Box<Node>,
        arguments: Vec<Node>
    },
    AttributeResolve {
        parent: Box<Node>,
        value: String
    },
    Program(Vec<Node>)
}

pub struct Parser<T: Iterator> {
    input: T
}

impl<T: Iterator<Item = TResult<Lexem>>> Parser<T> {
    pub fn new(input: T) -> Self {
        Self { input }
    }

    pub fn parse_function(&mut self) -> Option<Node> {
        todo!()
    }

    pub fn parse(&self) -> Node {
        let mut actions = Vec::<Node>::new();



        Node::Program(actions)
    }
}