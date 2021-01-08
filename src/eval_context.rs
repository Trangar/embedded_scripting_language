use crate::{Stack, Ast, Engine, MethodArgs, EvalOptions, AstStatement};
use alloc::vec::Vec;

pub struct EvalContext<'a, CB>
    where CB: for<'b> FnMut(&'a str, &mut MethodArgs<'b, 'a>, &mut Stack<'a>, &Ast<'a>) {

    pub ast: Ast<'a>,
    engine: &'a mut Engine<'a, CB>,
    stack: Vec<Stack<'a>>,
}

impl<'a, CB> EvalContext<'a, CB>
    where CB: for<'b> FnMut(&'a str, &mut MethodArgs<'b, 'a>, &mut Stack<'a>, &Ast<'a>) {
    pub fn new(engine: &'a mut Engine<'a, CB>, ast: Ast<'a>) -> Self {
        let mut stack = Vec::new();
        stack.push(Stack::default());
        Self {
            ast,
            engine,
            stack: stack,
        }
    }

    pub fn execute(&mut self, options: &EvalOptions) {
        if options.cycles == 0 {
            while self.is_running() {
                self.step();
            }
        } else {
            for _ in 0..options.cycles {
                if !self.is_running() { break;}
                self.step();
            }
        }
    }

    fn is_running(&self) -> bool {
        let index = self.stack.first().unwrap().ast_index;
        index < self.ast.steps.len()
    }

    fn step(&mut self) {
        let mut stack = self.stack.last_mut().unwrap();
        let stmt = &self.ast.steps[stack.ast_index];
        match &stmt.statement {
            AstStatement::Loop => {
                stack.ast_index += 1;
            },
            AstStatement::MethodCall {
                result_variable_name: _,
                name,
                args
            } => {
                let mut args = MethodArgs::from(args);
                (self.engine.cb)(
                    name, &mut args, stack, &self.ast
                );
                let next_ident = self.ast.steps.get(stack.ast_index + 1).map(
                    |step| step.ident
                ).unwrap_or(0);
                if next_ident < stmt.ident {
                    let (idx, ident_stmt) = self.ast.find_ident_stmt(stack.ast_index);
                    match &ident_stmt.statement {
                        AstStatement::Loop => {
                            stack.ast_index = idx + 1;
                        },
                        _ => {
                            self.stack.pop();
                            let mut stack = self.stack.last_mut().unwrap();
                            stack.ast_index += 1;
                        }
                    }
                } else {
                    stack.ast_index += 1;
                }
            }
        }
    }
}
