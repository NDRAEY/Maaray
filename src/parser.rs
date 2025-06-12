use std::{
    f32::consts::E,
    io::{Read, Seek},
};

use crate::{
    cursor::{self, VecCursor},
    lexer::{Lexem, LexemKind},
    tokenizer::{TResult, TokenKind},
};

#[derive(Debug, Clone)]
pub enum Node {
    Ident(String),
    Number(f64),
    String(String),
    Assignment {
        name: String,
        value: Box<Node>,
    },
    Function {
        name: String,
        arguments: Vec<Node>,
        code: Vec<Node>,
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
        arguments: Vec<Node>,
    },
    AttributeResolve {
        parent: Box<Node>,
        value: String,
    },
    If {
        condition: Box<Node>,
        alternative: Box<Node>,
        block: Box<Node>,
    },
    Program(Vec<Node>),
    Block(Vec<Node>),
}

pub struct Parser {
    input: cursor::VecCursor<Lexem>,
}

impl Parser {
    pub fn new(input: Vec<Lexem>) -> Self {
        Self {
            input: VecCursor::new(input),
        }
    }

    pub fn parse_ident(&mut self) -> Option<Node> {
        let nx = self.input.next();

        match nx {
            None => {
                self.input.prev();
                None
            }
            Some(x) => {
                let ret = x.ident().map(|a| Node::Ident(a.clone()));

                if ret.is_none() {
                    self.input.prev();
                }

                ret
            }
        }
    }

    pub fn parse_string(&mut self) -> Option<Node> {
        let nx = self.input.next();

        println!("String: {nx:?}");

        match nx {
            None => {
                self.input.prev();
                None
            }
            Some(x) => {
                if let LexemKind::StringLiteral(sl) = x.token() {
                    Some(Node::String(sl.clone()))
                } else {
                    self.input.prev();
                    None
                }
            }
        }
    }

    pub fn parse_number(&mut self) -> Option<Node> {
        let nx = self.input.next();

        println!("Number: {nx:?}");

        match nx {
            None => {
                self.input.prev();
                None
            }
            Some(x) => {
                if let LexemKind::Number(nr) = x.token() {
                    Some(Node::Number(*nr))
                } else {
                    self.input.prev();
                    None
                }
            }
        }
    }

    pub fn parse_block(&mut self) -> Option<Node> {
        let mut nodes = Vec::new();

        let lbrace = self.input.next().unwrap();

        if lbrace.token() != &LexemKind::LBrace {
            self.input.prev();

            return None;
        }

        loop {
            let value = self.parse();

            nodes.push(match value {
                Some(val) => val,
                None => break,
            });

            let rbrace = self.input.next().unwrap();

            if rbrace.token() != &LexemKind::RBrace {
                break;
            }
        }

        Some(Node::Block(nodes))
    }

    pub fn parse_comma_separated(&mut self) -> Option<Vec<Node>> {
        let mut values: Vec<Node> = Vec::new();

        loop {
            let value = self.parse_once();

            values.push(match value {
                Some(val) => val,
                None => break,
            });

            let comma = self.input.next().unwrap();

            if comma.token() != &LexemKind::Comma {
                self.input.prev();
                break;
            }
        }

        Some(values)
    }

    pub fn parse_function(&mut self) -> Option<Node> {
        let initial_position = self.input.position();
        let token = self.input.next().unwrap();

        if !token.is_ident_equals("func") {
            self.input.set_position(initial_position);

            return None;
        }

        let name = self.input.next().unwrap().ident();

        let name = match name {
            Some(value) => value.clone(),
            None => todo!("Implement Syntax error: expected function name!"),
        };

        println!("Parsing func with name: {name:?}");

        // If there's no `(`
        if self
            .input
            .next()
            .map(|a| a.token() != &LexemKind::LParen)
            .unwrap_or(false)
        {
            todo!("Implement Syntax error: expected `(`!");
        }

        let arguments = self.parse_comma_separated();
        println!("Args: {arguments:?}");

        // If there's no `)`
        if self
            .input
            .next()
            .map(|a| a.token() != &LexemKind::RParen)
            .unwrap_or(false)
        {
            todo!(
                "Implement Syntax error: expected `)`! Got: {:?}",
                self.input.current()
            );
        }

        let block = self.parse_block();
        println!("Name: {name:?}; Arguments: {arguments:?}; Code: {block:#?}");

        Some(Node::Function {
            name,
            arguments: arguments.unwrap(),
            code: {
                let Some(Node::Block(x)) = block else {
                    panic!("Expected block, but got None.");
                };

                x
            },
        })
    }

    pub fn parse_call(&mut self) -> Option<Node> {
        let initial_position = self.input.position();
        let name = self.parse_ident();

        let name = match name {
            Some(value) => value,
            None => {
                self.input.set_position(initial_position);
                return None;
            }
        };

        println!("call: {name:?}");

        // If there's no `(`
        if self
            .input
            .next()
            .map(|a| a.token() != &LexemKind::LParen)
            .unwrap_or(false)
        {
            self.input.set_position(initial_position);
            return None;
        }

        let parameters = self.parse_comma_separated();
        println!("call: parameters: {parameters:?}");

        // If there's no `)`
        if self
            .input
            .next()
            .map(|a| a.token() != &LexemKind::RParen)
            .unwrap_or(false)
        {
            todo!(
                "Implement Syntax error: expected `)`! ({:?}",
                self.input.current()
            );
        }

        Some(Node::Call {
            callee: Box::new(name),
            arguments: parameters.unwrap(),
        })
    }

    pub fn parse_expression(&mut self) -> Option<Node> {
        let current_node = self.parse_once();

        eprintln!("Current node is: {:?}", current_node);

        let parse1 = |this: &mut Self| {
            let next_lexem = this.input.next();

            if *next_lexem.unwrap().token() == LexemKind::Equals {
                // Should be `==`
                let next_lexem = this.input.next();
                if *next_lexem.unwrap().token() == LexemKind::Equals {
                    // It's `==`!

                    let node = this.parse_expression();

                    eprintln!("Node: {:?}", node);

                    return Some(Node::Equals(
                        Box::new(current_node.unwrap()),
                        Box::new(node.unwrap()),
                    ));
                }
            } else {
                // It seems it's a bare value
                this.input.prev();

                eprintln!("Bare value! {current_node:?}");

                if let Some(node) = current_node {
                    return Some(node);
                } else {
                    todo!("Parse other value from expression: {current_node:?}");
                }
            }

            todo!("Expression!")
        };

        let node = parse1(self);

        return node;
    }

    pub fn parse_if(&mut self) -> Option<Node> {
        let initial_position = self.input.position();
        let token = self.input.next().unwrap();

        if !token.is_ident_equals("if") {
            self.input.set_position(initial_position);

            return None;
        }

        let condition = self.parse_expression();

        eprintln!("{condition:#?}");

        eprintln!("{:?}", self.input.current());

        let block = self.parse_block();

        eprintln!("{block:?}");

        todo!("And what?")
    }

    pub fn parse_once(&mut self) -> Option<Node> {
        println!("? Block");
        if let Some(block) = self.parse_block() {
            println!("+ Block: {:?}", &block);
            return Some(block);
        }

        println!("? Condition");
        if let Some(condition) = self.parse_if() {
            println!("+ Condition: {:?}", &condition);
            return Some(condition);
        }

        println!("? Function");
        if let Some(func) = self.parse_function() {
            println!("+ Function: {:?}", &func);
            return Some(func);
        }

        println!("? Call");
        if let Some(call) = self.parse_call() {
            println!("+ Call: {:?}", &call);
            return Some(call);
        }

        println!("? Ident");
        if let Some(ident) = self.parse_ident() {
            println!("+ Ident: {:?}", &ident);
            return Some(ident);
        }

        println!("? String");
        if let Some(string) = self.parse_string() {
            println!("+ String: {:?}", &string);
            return Some(string);
        }

        println!("? Number");
        if let Some(number) = self.parse_number() {
            println!("+ Number: {:?}", &number);
            return Some(number);
        }

        let current_token_data = self
            .input
            .current()
            .map(|a| (a.token(), a.line(), a.column()))
            .unwrap();

        todo!(
            "Syntax error: Token: {:?}, Line: {:?}; Column: {:?}",
            current_token_data.0,
            current_token_data.1,
            current_token_data.2,
        );
    }

    pub fn parse(&mut self) -> Option<Node> {
        println!("= Entering parse");
        let mut actions = Vec::<Node>::new();

        while !self.input.reached_end() {
            let node = self.parse_once();

            actions.push(node.unwrap());
        }

        println!("= Exiting parse");

        Some(match actions.len() {
            1 => actions.pop().unwrap(),
            _ => Node::Program(actions),
        })
    }
}
