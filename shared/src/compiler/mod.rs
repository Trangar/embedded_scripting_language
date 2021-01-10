mod ast;
mod tokens;

pub fn compile(script: &str, buffer: &mut [u8]) -> Result<usize, Error> {
    use std::println;

    let mut tokens = tokens::tokenize(script);
    tokens::optimize(&mut tokens);

    let ast = ast::tokens_to_ast(&tokens);
    println!("{:#?}", ast);

    Ok(0)
}

#[derive(Debug)]
pub enum Error {}
