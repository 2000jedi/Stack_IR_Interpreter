use std::collections::LinkedList;

#[derive(Debug, Clone)]
pub enum Type {
    TInt, TFloat, TString, Void,
    TClass(String)
}

impl Type {
    pub fn from_string(s : String) -> Type {
        match s.as_str() {
            "int" => Type::TInt,
            "float" => Type::TFloat,
            "string" => Type::TString,
            "NULL" => Type::Void,
            _ => Type::TClass(s),
        }
    }
}

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

impl Atom {
    pub fn plus(&self, b : Atom) -> Atom {
        match self {
            Atom::VInt(l) => {
                match b {
                    Atom::VInt(r) => Atom::VInt(l + r),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            Atom::VFloat(l) => {
                match b {
                    Atom::VFloat(r) => Atom::VFloat(l + r),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            _ => panic! {
                "+: not implemented for {:?}", self
            }
        }
    }

    pub fn minus(&self, b : Atom) -> Atom {
        match self {
            Atom::VInt(l) => {
                match b {
                    Atom::VInt(r) => Atom::VInt(l - r),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            Atom::VFloat(l) => {
                match b {
                    Atom::VFloat(r) => Atom::VFloat(l - r),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            _ => panic! {
                "+: not implemented for {:?}", self
            }
        }
    }

    pub fn mult(&self, b : Atom) -> Atom {
        match self {
            Atom::VInt(l) => {
                match b {
                    Atom::VInt(r) => Atom::VInt(l * r),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            Atom::VFloat(l) => {
                match b {
                    Atom::VFloat(r) => Atom::VFloat(l * r),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            _ => panic! {
                "+: not implemented for {:?}", self
            }
        }
    }

    pub fn div(&self, b : Atom) -> Atom {
        match self {
            Atom::VInt(l) => {
                match b {
                    Atom::VInt(r) => Atom::VInt(l / r),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            Atom::VFloat(l) => {
                match b {
                    Atom::VFloat(r) => Atom::VFloat(l / r),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            _ => panic! {
                "+: not implemented for {:?}", self
            }
        }
    }

    pub fn rem(&self, b : Atom) -> Atom {
        match self {
            Atom::VInt(l) => {
                match b {
                    Atom::VInt(r) => Atom::VInt(l % r),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            _ => panic! {
                "+: not implemented for {:?}", self
            }
        }
    }

    pub fn gt(&self, b : Atom) -> Atom {
        match self {
            Atom::VInt(l) => {
                match b {
                    Atom::VInt(r) => Atom::VInt((*l > r) as i32),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            Atom::VFloat(l) => {
                match b {
                    Atom::VFloat(r) => Atom::VInt((*l > r) as i32),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            _ => panic! {
                "+: not implemented for {:?}", self
            }
        }
    }

    pub fn ge(&self, b : Atom) -> Atom {
        match self {
            Atom::VInt(l) => {
                match b {
                    Atom::VInt(r) => Atom::VInt((*l >= r) as i32),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            Atom::VFloat(l) => {
                match b {
                    Atom::VFloat(r) => Atom::VInt((*l >= r) as i32),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            _ => panic! {
                "+: not implemented for {:?}", self
            }
        }
    }

    pub fn lt(&self, b : Atom) -> Atom {
        match self {
            Atom::VInt(l) => {
                match b {
                    Atom::VInt(r) => Atom::VInt((*l < r) as i32),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            Atom::VFloat(l) => {
                match b {
                    Atom::VFloat(r) => Atom::VInt((*l < r) as i32),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            _ => panic! {
                "+: not implemented for {:?}", self
            }
        }
    }

    pub fn le(&self, b : Atom) -> Atom {
        match self {
            Atom::VInt(l) => {
                match b {
                    Atom::VInt(r) => Atom::VInt((*l <= r) as i32),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            Atom::VFloat(l) => {
                match b {
                    Atom::VFloat(r) => Atom::VInt((*l <= r) as i32),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            _ => panic! {
                "+: not implemented for {:?}", self
            }
        }
    }

    pub fn eq(&self, b : Atom) -> Atom {
        match self {
            Atom::VInt(l) => {
                match b {
                    Atom::VInt(r) => Atom::VInt((*l == r) as i32),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            Atom::VFloat(l) => {
                match b {
                    Atom::VFloat(r) => Atom::VInt((*l == r) as i32),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            Atom::VString(l) => {
                match b {
                    Atom::VString(r) => Atom::VInt((*l == r) as i32),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            _ => panic! {
                "+: not implemented for {:?}", self
            }
        }
    }

    pub fn ne(&self, b : Atom) -> Atom {
        match self {
            Atom::VInt(l) => {
                match b {
                    Atom::VInt(r) => Atom::VInt((*l != r) as i32),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            Atom::VFloat(l) => {
                match b {
                    Atom::VFloat(r) => Atom::VInt((*l != r) as i32),
                    _ => panic! {
                        "+: type mismatch!\nl={:?}\nr={:?}", self, b
                    }
                }
            }
            _ => panic! {
                "+: not implemented for {:?}", self
            }
        }
    }
}
