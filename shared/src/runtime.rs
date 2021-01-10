use crate::traits::State;

pub struct Runtime<'a, S: State> {
    pub bytecode: &'a mut [u8],
    pub state: S,
    pub program_counter: usize,
}

impl<'a, S: State> Runtime<'a, S> {
    pub fn new(bytecode: &'a mut [u8], state: S) -> Self {
        Self {
            bytecode,
            state,
            program_counter: 0,
        }
    }

    pub fn step(&mut self) {
        let instruction =
            crate::instructions::Instructions::get(&self.bytecode[self.program_counter..]);
        instruction.execute(self);
    }
}
