use std::collections::LinkedList;

#[derive(Debug, Clone)]
pub enum Atom {
    Ref(usize),
    VInt(i32),
    VFloat(f32),
    VString(String),
    Null,
}

#[derive(Debug)]
pub struct Memory {
    pub stack : LinkedList<Atom>,
    pub heap : Vec<Atom>
}

impl Memory {
    pub fn new() -> Memory {
        return Memory{stack: LinkedList::new(), heap: Vec::new()};
    }
}
