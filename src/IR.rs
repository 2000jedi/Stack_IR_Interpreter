use super::scanner::{Token, Scanner};
use super::mem_alloc::{Atom, Memory, Type};
use super::runtime;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Exec {
    pub tokens : Vec<Token>,
    pub labels : HashMap<usize, usize>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub class : Class,
    pub func : Fn,
}

#[derive(Debug, Clone)]
pub struct Class {
}

#[derive(Debug, Clone)]
pub struct DeFun {
    pub name : String,
    pub par_ts : Vec<Type>,
    pub ret_t  : Type,
    pub exec : Exec
}

#[derive(Debug, Clone)]
pub struct Fn {
    pub defs : Vec<DeFun>,
}

impl Exec {
    /**
     * JIT-compiler for the code block
     */
    pub fn compile(& self) {

    }

    pub fn simulate(& self, mut mem : Box<Memory>, prog : Box<Program>) -> Box<Memory> {
        let mut pc = 0;
        while pc < self.tokens.len() {
            match & self.tokens[pc] {
                Token::Pushi(val) => {
                    mem.stack.push_back(Atom::VInt(*val));
                }
                Token::Pushf(val) => {
                    mem.stack.push_back(Atom::VFloat(*val));
                }
                Token::Pop => {
                    if mem.stack.len() == 0 {
                        panic! {
                            "poping empty stack!"
                        }
                    }
                    mem.stack.pop_back();
                }
                Token::Store(iloc) => {
                    if mem.stack.len() == 0 {
                        panic! {
                            "poping empty stack!"
                        }
                    } else {
                        while *iloc >= mem.heap.len() {
                            mem.heap.push(Atom::Null);
                        }
                        mem.heap[*iloc] = mem.stack.pop_back().unwrap();
                    }
                }
                Token::Stores(iloc, string) => {
                    while *iloc >= mem.heap.len() {
                        mem.heap.push(Atom::Null);
                    }
                    mem.heap[*iloc] = Atom::VString(string.to_string());
                }
                Token::Load(iloc) => {
                    mem.stack.push_back(mem.heap[*iloc].clone());
                }
                Token::Call(name) => {
                    match name.as_str() {
                        "print" => {
                            let to_print = mem.stack.pop_back().unwrap();
                            mem = runtime::print(to_print, mem);
                        }
                        "println" => {
                            let to_print = mem.stack.pop_back().unwrap();
                            mem = runtime::print(to_print, mem);
                            println!();
                        }
                        "readint" => {
                            mem.stack.push_back(runtime::readint());
                        }
                        _ => {
                            let mut called = false;
                            for def in &prog.func.defs {
                                if def.name == *name {
                                    mem = def.simulate(mem, prog.clone());
                                    called = true;
                                    break;
                                }
                            }
                            if ! called {
                                panic!{
                                    "{}() no found", *name
                                }
                            }
                        }
                    }
                }
                Token::Dup => {
                    if mem.stack.len() == 0 {
                        panic!{
                            "stack is empty"
                        }
                    } else {
                        mem.stack.push_back(mem.stack.back().unwrap().clone());
                    }
                }
                Token::Label(_) => (),
                Token::Goto(lbl) => {
                    if ! self.labels.contains_key(lbl) {
                        panic! {
                            "label {} not found", lbl
                        }
                    } else {
                        pc = self.labels[lbl];
                    }
                }
                Token::Branch(lbl) => {
                    let br : usize;
                    if ! self.labels.contains_key(lbl) {
                        panic! {
                            "label {} not found", lbl
                        }
                    } else {
                        br = self.labels[lbl];
                    }
                    if mem.stack.len() == 0 {
                        panic! {
                            "poping empty stack!"
                        }
                    }
                    let z = mem.stack.pop_back().unwrap();
                    match z {
                        Atom::VInt(i) => {
                            if i != 0 {
                                pc = br;
                            }
                        }
                        _ => {
                            panic! {
                                "value {:?} does not have int type", z
                            }
                        }
                    }
                }
                Token::Add => {
                    let a = mem.stack.pop_back().unwrap();
                    let b = mem.stack.pop_back().unwrap();
                    mem.stack.push_back(a.plus(b));
                }
                Token::Sub => {
                    let a = mem.stack.pop_back().unwrap();
                    let b = mem.stack.pop_back().unwrap();
                    mem.stack.push_back(a.minus(b));
                }
                Token::Mul => {
                    let a = mem.stack.pop_back().unwrap();
                    let b = mem.stack.pop_back().unwrap();
                    mem.stack.push_back(a.mult(b));
                }
                Token::Div => {
                    let a = mem.stack.pop_back().unwrap();
                    let b = mem.stack.pop_back().unwrap();
                    mem.stack.push_back(a.div(b));
                }
                Token::Rem => {
                    let a = mem.stack.pop_back().unwrap();
                    let b = mem.stack.pop_back().unwrap();
                    mem.stack.push_back(a.rem(b));
                }
                Token::Eq => {
                    let a = mem.stack.pop_back().unwrap();
                    let b = mem.stack.pop_back().unwrap();
                    mem.stack.push_back(a.eq(b));
                }
                Token::Ne => {
                    let a = mem.stack.pop_back().unwrap();
                    let b = mem.stack.pop_back().unwrap();
                    mem.stack.push_back(a.ne(b));
                }
                Token::Lt => {
                    let a = mem.stack.pop_back().unwrap();
                    let b = mem.stack.pop_back().unwrap();
                    mem.stack.push_back(a.lt(b));
                }
                Token::Le => {
                    let a = mem.stack.pop_back().unwrap();
                    let b = mem.stack.pop_back().unwrap();
                    mem.stack.push_back(a.le(b));
                }
                Token::Gt => {
                    let a = mem.stack.pop_back().unwrap();
                    let b = mem.stack.pop_back().unwrap();
                    mem.stack.push_back(a.gt(b));
                }
                Token::Ge => {
                    let a = mem.stack.pop_back().unwrap();
                    let b = mem.stack.pop_back().unwrap();
                    mem.stack.push_back(a.ge(b));
                }
                _ => {
                    unreachable!();
                }
            }

            pc += 1;
        }
        return mem;
    }
}

impl DeFun {
    pub fn simulate(& self, mut mem : Box<Memory>, prog : Box<Program>) -> Box<Memory> {
        mem = self.exec.simulate(mem, prog);
        return mem;
    }
}

impl Program {
    pub fn simulate(&self, mut mem : Box<Memory>) {
        for fns in & self.func.defs {
            if fns.name == "main" {
                mem = fns.simulate(mem, Box::new(self.clone()));
            }
        }
    }
}

pub fn make_class(mut scan: Scanner) -> (Class, Scanner) {
    match scan.next() {
        Some(v) => {
            match v.token {
                Token::SClass => {
                    return (Class{}, scan);
                    // TODO: class construct
                }
                _ => {
                    panic!{
                        "{}: expected SClass, found {:?}", 
                        v.row, v.token
                    }
                }
            }
        }
        None => {
            panic!{
                "expected SClass, found EOL"
            }
        }
    }
}

pub fn make_execs(mut scan: Scanner) -> (Exec, Scanner) {
    let mut execs = Exec{tokens: vec![], labels: HashMap::new()};
    loop {
        match scan.peek() {
            Some(v) => {
                match v.token {
                    Token::Endef => {
                        break;
                    }
                    Token::Label(lbl) => {
                        execs.labels.insert(lbl, execs.tokens.len());
                        execs.tokens.push(v.token);
                        scan.next();
                    }
                    _ => {
                        execs.tokens.push(v.token);
                        scan.next();
                    }
                }
            }
            None => {
                break;
            }
        }
    }
    return (execs, scan);
}

pub fn make_defun(mut scan: Scanner) -> 
        (Option<DeFun>, Scanner) {
    match scan.peek() {
        Some(v) => {
            match v.token {
                Token::Defun(name, par_ts, ret_t) => {
                    scan.next();
                    let (exec, mut scan) = make_execs(scan);
                    match scan.next() {
                        Some(v) => {
                            match v.token {
                                Token::Endef => {}
                                _ => {
                                    panic! {
                                        "{}: expected Endef, found {:?}", v.row, v.token
                                    }
                                }
                            }
                        }
                        None => {
                            panic! {
                                "expected Endef, found EOF"
                            }
                        }
                    }
                    return (Some(DeFun{name, par_ts, ret_t, exec}), scan);
                }
                _ => {
                    return (None, scan);
                }
            }
        }
        None => {
            return (None, scan);
        }
    }
}

pub fn make_fn(mut scan: Scanner) -> (Fn, Scanner) {
    match scan.next() {
        Some(v) => {
            match v.token {
                Token::SFn => {
                    let mut defs : Vec<DeFun> = vec![];
                    loop {
                        let (defun, scan_) = make_defun(scan);
                        scan = scan_;
                        match defun {
                            Some(defn) => {
                                defs.push(defn);
                            }
                            None => {
                                break;
                            }
                        }
                    }
                    return (Fn{defs}, scan);
                }
                _ => {
                    panic! {
                        "{}: expected SFn, found {:?}", v.row, v.token
                    }
                }
            }
        }
        None => {
            panic! {
                "expected SFn, found EOL"
            }
        }
    }
}

pub fn make_ir(mut scan : Scanner) -> Program {
    match scan.next() {
        Some(prog) => {
            match prog.token {
                Token::SRaw => {
                    let (class, scan) = make_class(scan);
                    let (func, _) = make_fn(scan);
                    return Program{class, func};
                }
                _ => {
                    panic! {
                        "{}: expected SRaw, found {:?}", prog.row, prog.token
                    }
                }
            }
        }
        None => {
            panic! {
                "file is empty"
            }
        }
    }
}
