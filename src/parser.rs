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
        code: Box<Node>,
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
        value: Box<Node>,
    },
    If {
        condition: Box<Node>,
        alternative: Box<Node>,
        block: Box<Node>,
    },
    Program(Vec<Node>),
    Block(Vec<Node>),
}

impl Node {
    pub fn ident(&self) -> Option<&String> {
        if let Node::Ident(id) = self {
            return Some(id);
        }

        None
    }
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
        let current_position = self.input.position();

        let lbrace = self.input.next().unwrap();

        if lbrace.token() != &LexemKind::LBrace {
            self.input.prev();

            return None;
        }

        println!("LBrace passed");

        let value = self.parse();

        println!("Parsed: {:#?}", value);

        let rbrace = self.input.next().unwrap();

        if rbrace.token() != &LexemKind::RBrace {
            self.input.set_position(current_position);

            return None;
        }

        Some(value)
    }

    pub fn parse_comma_separated(&mut self) -> Option<Vec<Node>> {
        let mut values: Vec<Node> = Vec::new();

        loop {
            let value = self.parse_expression();

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
                let Some(x) = block else {
                    panic!("Expected block, but got None.");
                };

                Box::new(x)
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

    pub fn parse_expression_level_1(&mut self) -> Option<Node> {
        let current_node = self.parse_chained();

        eprintln!("Current node is: {:?}", current_node);

        let next_lexem = self.input.next();

        if *next_lexem.unwrap().token() == LexemKind::Equals {
            // Should be `==`
            let next_lexem = self.input.next();
            if *next_lexem.unwrap().token() == LexemKind::Equals {
                // It's `==`!

                let node = self.parse_expression_level_1();

                eprintln!("Node: {:?}", node);

                return Some(Node::Equals(
                    Box::new(current_node.unwrap()),
                    Box::new(node.unwrap()),
                ));
            }
        } else {
            // It seems it's a bare value
            self.input.prev();

            eprintln!("Bare value! {current_node:?}");

            // if let Some(node) = current_node {
            //     return Some(node);
            // } else {
            //     todo!("Parse other value from expression: {current_node:?}");
            // }

            return current_node;
        }

        todo!("Expression!")
    }

    pub fn parse_expression(&mut self) -> Option<Node> {
        let lhs = self.parse_expression_level_1();

        let after = self.input.next()?;

        if after.token() == &LexemKind::Or {
            let next = self.input.next()?;

            if next.token() == &LexemKind::Or {
                let rhs = self.parse_expression_level_1();

                // println!("LHS: {lhs:?}");
                // println!("RHS: {rhs:?}");

                // todo!("WTF");

                return Some(Node::Or(Box::new(lhs.unwrap()), Box::new(rhs.unwrap())));
            }
        } else if after.token() == &LexemKind::Plus {
            let rhs = self.parse_expression_level_1();

            return Some(Node::Add(Box::new(lhs.unwrap()), Box::new(rhs.unwrap())));
        } else if after.token() == &LexemKind::Minus {
            let rhs = self.parse_expression_level_1();

            return Some(Node::Subtract(
                Box::new(lhs.unwrap()),
                Box::new(rhs.unwrap()),
            ));
        } else {
            self.input.prev();
        }

        return lhs;
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

        eprintln!("Block: {block:?}");

        return Some(Node::If {
            condition: Box::new(condition.unwrap()),
            alternative: Box::new(Node::Program(Vec::new())),
            block: Box::new(block.unwrap()),
        });
    }

    pub fn parse_return(&mut self) -> Option<Node> {
        let token = self.input.next();
        let is_return_token = token
            .map(|a| a.ident().map(|x| x == "return").unwrap_or(false))
            .unwrap_or(false);

        if !is_return_token {
            self.input.prev();

            return None;
        }

        let expression = self.parse_expression();

        eprintln!("Expression: {expression:#?}");

        self.consume_semicolon();

        return Some(Node::Return(Box::new(expression.unwrap())));
    }

    pub fn parse_declaration(&mut self) -> Option<Node> {
        let position = self.input.position();

        let token = self.input.next().cloned();
        let is_let_token = token
            .as_ref()
            .map(|a| a.ident().map(|x| x == "let").unwrap_or(false))
            .unwrap_or(false);

        if !is_let_token {
            self.input.prev();

            return None;
        }

        let name = self.parse_ident();

        let sign = self.input.next().cloned();
        let is_equal_sign = sign
            .as_ref()
            .map(|x| x.token() == &LexemKind::Equals)
            .unwrap_or(false);

        if !is_equal_sign {
            self.input.set_position(position);
        }

        let expression = self.parse_expression();

        eprintln!("Name: {name:?}; Sign: {sign:?}; Expression: {expression:#?}");

        self.consume_semicolon();

        // todo!("Is assignment stable?");

        return Some(Node::Assignment {
            name: name.map(|x| x.ident().cloned().unwrap()).unwrap(),
            value: Box::new(expression.unwrap()),
        });
    }

    pub fn parse_attr_resolve(&mut self) -> Option<Node> {
        // let position = self.input.position();

        let object = self.parse_atom();
        let token = self.input.next();

        if !token.map(|x| x.token() == &LexemKind::Dot).unwrap_or(false) {
            self.input.prev();

            return object;
        }

        let attr = self.parse_chained();

        return Some(Node::AttributeResolve {
            parent: Box::new(object.unwrap()),
            value: Box::new(attr.unwrap()),
        });
    }

    pub fn parse_chained(&mut self) -> Option<Node> {
        println!("? Attribute resolve");
        if let Some(res) = self.parse_attr_resolve() {
            println!("+ Attribute resolve: {:?}", &res);
            return Some(res);
        }

        return self.parse_atom();
    }

    pub fn parse_atom(&mut self) -> Option<Node> {
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

        return None;
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

        println!("? Declaration");
        if let Some(decl) = self.parse_declaration() {
            println!("+ Declaration: {:?}", &decl);
            return Some(decl);
        }

        println!("? Return");
        if let Some(ret) = self.parse_return() {
            println!("+ Return: {:?}", &ret);
            return Some(ret);
        }

        println!("? Expression");
        if let Some(expr) = self.parse_expression() {
            println!("+ Expression: {:?}", &expr);
            return Some(expr);
        }

        println!("? Block exit");
        if self
            .input
            .current()
            .map(|x| x.token() == &LexemKind::RBrace)
            .unwrap_or(false)
        {
            return None;
        }

        self.consume_semicolon();

        let current_token_data = self
            .input
            .current()
            .map(|a| (a.token(), a.line(), a.column()));

        if let Some(data) = current_token_data {
            todo!(
                "Syntax error: Token: {:?}, Line: {:?}; Column: {:?}",
                data.0,
                data.1,
                data.2,
            );
        }

        return None;
    }

    pub fn parse(&mut self) -> Node {
        println!("= Entering parse");
        let mut actions: Vec<Node> = Vec::new();

        while !self.input.reached_end() {
            let node = self.parse_once();

            if node.is_none() {
                break;
            }

            actions.push(node.unwrap());
        }

        println!("= Exiting parse");

        match actions.len() {
            1 => actions.pop().unwrap(),
            _ => Node::Program(actions),
        }
    }

    fn consume_semicolon(&self) -> bool {
        let token = self.input.next();

        token
            .map(|a| a.token() == &LexemKind::Semicolon)
            .unwrap_or(false)
    }
}
