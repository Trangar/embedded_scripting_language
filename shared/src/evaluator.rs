use crate::instructions::Instructions;

pub struct Evaluator<'a> {
    buffer: &'a [u8],
    index: usize,
    memory: &'a mut [u8],
}

impl<'a> Evaluator<'a> {
    pub fn evaluate(&mut self) {
        let instruction = Instructions::get(&self.buffer[self.index..]);
    }
}
