use crate::{Ast, EvalContext};
use crate::{MethodArgs, Stack};
use core::marker::PhantomData;

pub struct Engine<'a, CB> 
    where CB: for<'b> FnMut(&'a str, &mut MethodArgs<'b, 'a>, &mut Stack<'a>, &Ast<'a>) {
    pub cb: CB,
    pd: PhantomData<&'a ()>,
}

impl<'a, CB> Engine<'a, CB>
    where CB: for<'b> FnMut(&'a str, &mut MethodArgs<'b, 'a>, &mut Stack<'a>, &Ast<'a>) {
    pub fn new(cb: CB) -> Self {
        Self {
            cb,
            pd: PhantomData,
        }
    }

    pub fn start_eval(&'a mut self, script: &'a str) -> EvalContext<'a, CB> {
        let ast = Ast::parse(script);
        EvalContext::new(self, ast)
    }
}
