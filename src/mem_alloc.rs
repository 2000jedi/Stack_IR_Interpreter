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
    pub stack : Vec<Atom>,
    pub heap : Vec<Atom>
}

impl Memory {
    pub fn new() -> Memory {
        return Memory{stack: Vec::new(), heap: Vec::new()};
    }
}
