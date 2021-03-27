use super::mem_alloc::*;

pub fn print(mut atom : Atom, mut mem : Box<Memory>) -> Box<Memory> {
    match atom {
        Atom::Ref(r) => {
            atom = mem.heap[r].clone();
            mem = print(atom, mem);
        }
        Atom::VInt(val) => {
            println!("{}", val);
        }
        Atom::VFloat(val) => {
            println!("{}", val);
        }
        Atom::VString(val) => {
            println!("{}", val);
        }
        Atom::Null => {
            println!("Null");
        }
    }
    return mem;
}
