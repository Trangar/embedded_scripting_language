use alloc::vec::Vec;

#[derive(Debug)]
pub struct MethodArgs<'a, 'b> {
    args: &'a [&'b str],
}

impl<'a, 'b> MethodArgs<'a, 'b> {
    pub fn next(&mut self) -> &'b str {
        let next_arg = self.args[0];
        self.args = &self.args[1..];
        next_arg
    }
}

impl<'a, 'b> From<&'a Vec<&'b str>> for MethodArgs<'a, 'b> {
    fn from(args: &'a Vec<&'b str>) -> Self {
        Self {
            args: args.as_slice(),
        }
    }
}