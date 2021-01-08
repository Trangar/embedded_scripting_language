use alloc::boxed::Box;
use core::any::Any;
use hashbrown::hash_map::{HashMap, Entry};

#[derive(Default)]
pub struct Stack<'a> {
    pub ast_index: usize,
    pub variables: HashMap<&'a str, Box<dyn Any>>,
}

impl<'a> Stack<'a> {
    pub fn set<T: 'static>(&mut self, name: &'a str, value: T) {
        let entry = self.variables.entry(name);
        match entry {
            Entry::Vacant(vacant) => {
                vacant.insert(Box::new(value) as Box<dyn Any>);
            },
            Entry::Occupied(mut val) => {
                val.insert(Box::new(value) as Box<dyn Any>);
            }
        }
    }

    pub fn get<T: 'static>(&mut self, name: &'a str) -> &T {
        self.variables[name].downcast_ref::<T>().unwrap()
    }
    pub fn take<T: 'static>(&mut self, name: &'a str) -> Box<T> {
        self.variables.remove(name).unwrap().downcast::<T>().unwrap()
    }
}

pub struct StackVariable<'a> {
    name: &'a str,
    value: Box<dyn Any>,
}

impl<'a> StackVariable<'a> {
    pub fn create<T>(scope: &mut Stack, name: &'a str, value: T) -> Self
    where
        T: 'static,
    {
        Self {
            name,
            value: Box::new(value) as Box<dyn Any>,
        }
    }
}
