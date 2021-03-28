use std::io::Write;
use super::mem_alloc::*;

pub fn print(mut atom : Atom, mut mem : Box<Memory>) -> Box<Memory> {
    match atom {
        Atom::Ref(r) => {
            atom = mem.heap[r].clone();
            mem = print(atom, mem);
        }
        Atom::VInt(val) => {
            print!("{}", val);
        }
        Atom::VFloat(val) => {
            print!("{}", val);
        }
        Atom::VString(val) => {
            print!("{}", val);
        }
        Atom::Null => {
            print!("Null");
        }
    }
    std::io::stdout().flush().unwrap();
    return mem;
}

pub fn readint() -> Atom {
    return Atom::VInt(read!());
}
