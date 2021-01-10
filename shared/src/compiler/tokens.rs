use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Number(i32),
    Word(&'a str),
    Ident(u8),
    BananaOpen,
    EndStatement,
    BananaClose,
    Comma,
    Loop,
    For,
    If,
    In,
    And,
    Or,
    Not,
    Equals,
    Assign,
    Colon,
    Multiply,
    Plus,
    Minus,
    LessThan,
    LessOrEqualTo,
    GreaterThan,
    GreaterOrEqualTo,
}

pub fn tokenize(mut script: &str) -> Vec<Token> {
    let mut result = Vec::new();

    'outer: while !script.is_empty() {
        if script.starts_with("    ") {
            let mut ident_count = 0;
            while script.starts_with("    ") {
                ident_count += 1;
                script = &script[4..];
            }
            result.push(Token::Ident(ident_count));
        }
        if script.starts_with('\t') {
            let mut ident_count = 0;
            while script.starts_with('\t') {
                ident_count += 1;
                script = &script[1..];
            }
            result.push(Token::Ident(ident_count));
        }
        while let Some(index) = script.bytes().position(|c| {
            [
                b'\t', b'\n', b'\r', b'(', b')', b'=', b':', b'*', b'>', b'<', b'+', b',', b'-',
                b' ',
            ]
            .contains(&c)
        }) {
            if index == 0 {
                let c = script.bytes().next().unwrap() as char;
                let mut chars_taken = 1;
                match c {
                    ' ' => {}
                    '\n' => {
                        result.push(Token::EndStatement);
                        script = &script[1..];
                        continue 'outer;
                    }
                    '=' => {
                        if let Some(b'=') = script.bytes().nth(1) {
                            result.push(Token::Equals);
                            chars_taken += 1;
                        } else {
                            result.push(Token::Assign);
                        }
                    }
                    '>' => {
                        if let Some(b'=') = script.bytes().nth(1) {
                            result.push(Token::GreaterOrEqualTo);
                            chars_taken += 1;
                        } else {
                            result.push(Token::GreaterThan);
                        }
                    }
                    '<' => {
                        if let Some(b'=') = script.bytes().nth(1) {
                            result.push(Token::LessOrEqualTo);
                            chars_taken += 1;
                        } else {
                            result.push(Token::LessThan);
                        }
                    }
                    '(' => result.push(Token::BananaOpen),
                    ')' => result.push(Token::BananaClose),
                    ',' => result.push(Token::Comma),
                    ':' => result.push(Token::Colon),
                    '*' => result.push(Token::Multiply),
                    '+' => result.push(Token::Plus),
                    '-' => result.push(Token::Minus),
                    x => panic!("Unknown token: {:?}", x),
                }
                script = &script[chars_taken..];
            } else {
                let word = &script[..index].trim();
                if let Ok(num) = word.parse() {
                    result.push(Token::Number(num));
                } else {
                    match *word {
                        "loop" => result.push(Token::Loop),
                        "for" => result.push(Token::For),
                        "in" => result.push(Token::In),
                        "if" => result.push(Token::If),
                        "and" => result.push(Token::And),
                        "or" => result.push(Token::Or),
                        "not" => result.push(Token::Not),
                        word => result.push(Token::Word(word)),
                    }
                }
                script = &script[index..]
            }
        }
    }

    result
}

pub fn optimize(tokens: &mut Vec<Token>) {
    let mut idx = 0;
    'optimize_loop: while idx < tokens.len() {
        if idx > 0 && [Token::Multiply, Token::Minus, Token::Plus].contains(&tokens[idx]) {
            if let (Some(Token::Number(left)), Some(Token::Number(right))) =
                (tokens.get(idx - 1), tokens.get(idx + 1))
            {
                let result = match &tokens[idx] {
                    Token::Multiply => left * right,
                    Token::Minus => left - right,
                    Token::Plus => left + right,
                    token => panic!("Unknown math optimization: {:?}", token),
                };
                tokens[idx - 1] = Token::Number(result);
                tokens.remove(idx);
                tokens.remove(idx);
                continue 'optimize_loop;
            }
        }
        if tokens[idx] == Token::EndStatement {
            while let Some(Token::EndStatement) = tokens.get(idx + 1) {
                tokens.remove(idx + 1);
            }
        }
        if let Token::Ident(num) = tokens[idx] {
            match tokens.get(idx + 1) {
                Some(Token::EndStatement) => {
                    tokens.remove(idx);
                    continue 'optimize_loop;
                }
                Some(Token::Ident(num2)) => {
                    tokens[idx] = Token::Ident(num + num2);
                    tokens.remove(idx + 1);
                    continue 'optimize_loop;
                }
                _ => {}
            }
        }
        idx += 1;
    }
    while let Some(Token::EndStatement) = tokens.first() {
        tokens.remove(0);
    }
    while let Some(Token::EndStatement) = tokens.last() {
        tokens.pop();
    }
    tokens.push(Token::EndStatement);
}
