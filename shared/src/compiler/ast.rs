use super::tokens::Token;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ops::Range;
use std::println;

#[derive(Debug)]
pub enum Ast<'a> {
    ConstantNum(i32),
    Variable {
        name: &'a str,
    },
    Assign {
        var_name: &'a str,
        rhs: Box<Ast<'a>>,
    },
    Method {
        method_name: &'a str,
        args: Vec<Ast<'a>>,
    },
    Expression {
        left: &'a str,
        operation: Operation,
        right: Box<Ast<'a>>,
    },
    Loop {
        statements: Vec<Ast<'a>>,
    },
    For {
        range: Range<i32>,
        var_name: &'a str,
        statements: Vec<Ast<'a>>,
    },
    Block {
        statements: Vec<Ast<'a>>,
    },
}

#[derive(Debug)]
pub enum Operation {
    Minus,
    Plus,
    Multiply,
}

impl Operation {
    pub fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Minus => Some(Self::Minus),
            Token::Plus => Some(Self::Plus),
            Token::Multiply => Some(Self::Multiply),
            _ => None,
        }
    }
}

pub fn tokens_to_ast<'a>(mut tokens: &[Token<'a>]) -> Ast<'a> {
    let result = parse_block(&mut tokens, 0, false);
    println!("{:#?}", result);
    println!("{:?}", tokens);
    assert!(tokens.is_empty());

    Ast::Block { statements: result }
}

fn parse_block<'a>(tokens: &mut &[Token<'a>], ident: u8, nested: bool) -> Vec<Ast<'a>> {
    let mut result = Vec::<Ast<'a>>::new();
    let mut last_len = tokens.len() + 1;
    while !tokens.is_empty() {
        if cfg!(debug_assertions) && last_len == tokens.len() {
            panic!("Infinite loop detected, aborting\n{:?}", tokens);
        }
        last_len = tokens.len();

        let (first, second) = (tokens.get(0), tokens.get(1));
        match (first, second) {
            (Some(Token::Ident(n)), _) => {
                if *n == ident {
                    // same block
                    *tokens = &tokens[1..];
                } else if n + 1 == ident {
                    // end block
                    break;
                } else {
                    panic!("Unknown ident: {}, expected {} or {}", n, ident - 1, ident);
                }
            }
            (Some(Token::Word(method_name)), Some(Token::BananaOpen)) => {
                *tokens = &tokens[2..];
                let args = parse_method_args(tokens);
                result.push(Ast::Method { method_name, args });
                if nested {
                    break;
                }
                assert_eq!(tokens.first(), Some(&Token::EndStatement));
                *tokens = &tokens[1..];
            }
            (Some(Token::Number(num)), Some(Token::EndStatement))
                if nested && result.is_empty() =>
            {
                result.push(Ast::ConstantNum(*num));
                *tokens = &tokens[1..];
                return result;
            }
            (Some(Token::Word(var_name)), Some(Token::Assign)) => {
                *tokens = &tokens[2..];
                let mut rhs = parse_block(tokens, u8::max_value(), true);
                let rhs = if rhs.len() == 1 {
                    Box::new(rhs.pop().unwrap())
                } else {
                    Box::new(Ast::Block { statements: rhs })
                };
                result.push(Ast::Assign { var_name, rhs });
                assert_eq!(tokens.first(), Some(&Token::EndStatement));
                *tokens = &tokens[1..];
            }
            (Some(Token::Loop), Some(Token::Colon)) => {
                *tokens = &tokens[2..];
                assert_eq!(tokens.first(), Some(&Token::EndStatement));
                *tokens = &tokens[1..];
                let statements = parse_block(tokens, ident + 1, false);
                result.push(Ast::Loop { statements });
            }
            (Some(Token::If), _) => {
                *tokens = &tokens[1..];
                let statements = parse_block(tokens, u8::max_value(), false);
                panic!("If statements: {:?}", statements);
            }
            (Some(Token::For), Some(Token::Word(var_name))) => {
                *tokens = &tokens[2..];
                assert_eq!(tokens.first(), Some(&Token::In));
                *tokens = &tokens[1..];
                let (start, end) = match &tokens[..5] {
                    [Token::Number(start), Token::Comma, Token::Number(end), Token::Colon, Token::EndStatement] => {
                        (*start, *end)
                    }
                    tokens => panic!(
                        "Expected `for {} in <start>,<end>:`, got `for {} in {:?}`",
                        var_name, var_name, tokens
                    ),
                };
                *tokens = &tokens[5..];
                let statements = parse_block(tokens, ident + 1, false);
                result.push(Ast::For {
                    range: start..end,
                    var_name,
                    statements,
                });
            }
            (Some(Token::For), stmt) => {
                panic!("Malformed for statement, expected word, got {:?}", stmt);
            }
            (Some(Token::Word(var)), Some(t)) if Operation::from_token(t).is_some() => {
                let op = Operation::from_token(t).unwrap();
                *tokens = &tokens[2..];
                let (token_length, rhs) = match &tokens {
                    &[Token::Number(n)] => (1, Ast::ConstantNum(*n)),
                    &[Token::Word(word)] => (1, Ast::Variable { name: word }),
                    t if t.len() > 1 => (
                        0,
                        Ast::Block {
                            statements: parse_block(tokens, u8::max_value(), true),
                        },
                    ),
                    _ => panic!("Unterminated statement: {:?} {:?}", var, op),
                };
                result.push(Ast::Expression {
                    left: var,
                    operation: op,
                    right: Box::new(rhs),
                });
                *tokens = &tokens[token_length..];
            }

            (first, second) => {
                println!("{:#?}", result);
                panic!("Unknown combination: {:?} + {:?}", first, second);
            }
        }
    }

    result
}

fn parse_method_args<'a>(tokens: &mut &[Token<'a>]) -> Vec<Ast<'a>> {
    let mut args = Vec::new();

    while !tokens.is_empty() {
        let next_indicator_index = tokens
            .iter()
            .position(|t| matches!(t, Token::BananaClose | Token::BananaOpen | Token::Comma))
            .unwrap();
        let next_indicator = &tokens[next_indicator_index];
        if let Token::BananaOpen = next_indicator {
            // nested method call
            assert_eq!(1, next_indicator_index);
            let method_name = match &tokens[0] {
                Token::Word(name) => name,
                x => panic!("Expected method name, got {:?}", x),
            };
            *tokens = &tokens[2..];
            let nested_args = parse_method_args(tokens);
            args.push(Ast::Method {
                method_name,
                args: nested_args,
            });
        } else if next_indicator_index == 0 {
            *tokens = &tokens[1..];
            break;
        } else if next_indicator_index == 1 {
            if let Token::Number(num) = tokens[0] {
                args.push(Ast::ConstantNum(num));
            } else if let Token::Word(name) = tokens[0] {
                args.push(Ast::Variable { name });
            } else {
                panic!("Unexpected function argument: {:?}", tokens[0]);
            }
        } else {
            let mut nested_tokens = &tokens[..next_indicator_index];
            let statements = parse_block(&mut nested_tokens, u8::max_value(), true);
            args.push(Ast::Block { statements })
        }
        *tokens = &tokens[next_indicator_index + 1..];
        if let Token::BananaClose = next_indicator {
            break;
        }
    }
    args
}
