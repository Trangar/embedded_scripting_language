use alloc::vec::Vec;
use crate::Stack;

#[derive(Debug)]
pub struct Ast<'a> {
    pub steps: Vec<AstStep<'a>>,
}

#[derive(Debug)]
pub struct AstStep<'a> {
    pub ident: u8,
    pub statement: AstStatement<'a>,

}

#[derive(Debug)]
pub enum AstStatement<'a> {
    MethodCall {
        result_variable_name: Option<&'a str>,
        name: &'a str,
        args: Vec<&'a str>,
    },
    Loop,
}

impl<'a> Ast<'a> {
    pub fn parse(s: &'a str) -> Self {
        let mut steps = Vec::new();
        for line in s.lines() {
            let (mut line, ident) = get_ident(line);
            if line.trim() == "loop:" {
                steps.push(AstStep {
                    ident,
                    statement: AstStatement::Loop,
                });
                continue;
            }

            let (maybe_left, maybe_right) = {
                let mut split = line.splitn(2, '=');
                (split.next(), split.next())
            };

            let result_variable_name = if let (Some(left), Some(right)) = (maybe_left, maybe_right)
            {
                line = right.trim();
                Some(left.trim())
            } else {
                None
            };

            if let Some(idx) = line.chars().position(|c| c == '(') {
                let (fn_name, remaining) = line.split_at(idx);
                let remaining = remaining[1..].trim_end_matches(')');
                let args = remaining.split(',').map(|a| a.trim()).collect();
                steps.push(AstStep {
                    ident,
                    statement: AstStatement::MethodCall {
                        result_variable_name,
                        name: fn_name.trim(),
                        args,
                    },
                });
                continue;
            }
        }

        Self {
            steps
        }
    }

    pub fn get_return_variable_name(&self, scope: &mut Stack) -> Option<&'a str> {
        match self.steps[scope.ast_index].statement {
            AstStatement::MethodCall { result_variable_name, .. } => result_variable_name,
            _ => None
        }
    }

    pub fn find_ident_stmt(&self, mut index: usize) -> (usize, &AstStep) {
        let ident = self.steps[index].ident;
        index -= 1;
        while self.steps[index].ident == ident {
            index -= 1;
        }
        (index, &self.steps[index])
    }
}

fn get_ident(mut line: &str) -> (&str, u8) {
    let mut ident = 0;
    while let Some(new_line) = line.strip_prefix("    ") {
        line = new_line;
        ident += 1;
    }
    while let Some(new_line) = line.strip_prefix('\t') {
        line = new_line;
        ident += 1;
    }

    (line, ident)
}
