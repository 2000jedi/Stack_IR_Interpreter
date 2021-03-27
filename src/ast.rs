use super::scanner::{Token, Scanner};
use super::mem_alloc::{Atom, Memory};
use super::runtime;

#[derive(Debug, Clone)]
pub struct Exec {
    pub tokens : Vec<Token>,
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
    pub pars : usize,
    pub exec : Exec
}

#[derive(Debug, Clone)]
pub struct Fn {
    pub defs : Vec<DeFun>,
}

impl Exec {
    pub fn compile(& self) {

    }

    pub fn simulate(& self, mut mem : Box<Memory>, prog : Box<Program>) -> Box<Memory> {
        for token in & self.tokens {
            match token {
                Token::Pushi(val) => {
                    mem.stack.push(Atom::VInt(*val));
                }
                Token::Pushf(val) => {
                    mem.stack.push(Atom::VFloat(*val));
                }
                Token::Stores(iloc, string) => {
                    while *iloc >= mem.heap.len() {
                        mem.heap.push(Atom::Null);
                    }
                    mem.heap[*iloc] = Atom::VString(string.to_string());
                }
                Token::Load(iloc) => {
                    mem.stack.push(Atom::Ref(*iloc));
                }
                Token::Call(name) => {
                    match name.as_str() {
                        "print" => {
                            let to_print = mem.stack.pop().unwrap();
                            mem = runtime::print(to_print, mem);
                        }
                        _ => {
                            for def in &prog.func.defs {
                                if def.name == *name {
                                    mem = def.simulate(mem, prog.clone());
                                }
                            }
                        }
                    }
                }
                _ => {
                    unreachable!();
                }
            }
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
                    eprintln!("{}: expected SClass, found {:?}", 
                        v.row, v.token);
                    std::process::exit(super::COMPILE_ERROR);
                }
            }
        }
        None => {
            eprintln!("expected SClass, found EOL");
            std::process::exit(super::COMPILE_ERROR);
        }
    }
}

pub fn make_execs(mut scan: Scanner) -> (Exec, Scanner) {
    let mut execs = Exec{tokens: vec![]};
    loop {
        match scan.peek() {
            Some(v) => {
                match v.token {
                    Token::Endef => {
                        break;
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
                Token::Defun(name, pars) => {
                    scan.next();
                    let (exec, mut scan) = make_execs(scan);
                    match scan.next() {
                        Some(v) => {
                            match v.token {
                                Token::Endef => {}
                                _ => {
                                    eprintln!("{}: expected Endef, found {:?}",
                                        v.row, v.token);
                                    std::process::exit(super::COMPILE_ERROR);
                                }
                            }
                        }
                        None => {
                            eprintln!("expected Endef, found EOL");
                            std::process::exit(super::COMPILE_ERROR);
                        }
                    }
                    return (Some(DeFun{name, pars, exec}), scan);
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
                    eprintln!("{}: expected SFn, found {:?}", 
                        v.row, v.token);
                    std::process::exit(super::COMPILE_ERROR);
                }
            }
        }
        None => {
            eprintln!("expected SFn, found EOL");
            std::process::exit(super::COMPILE_ERROR);
        }
    }
}

pub fn make_ast(mut scan : Scanner) -> Program {
    match scan.next() {
        Some(prog) => {
            match prog.token {
                Token::SRaw => {
                    let (class, scan) = make_class(scan);
                    let (func, _) = make_fn(scan);
                    return Program{class, func};
                }
                _ => {
                    eprintln!("{}: expected SRaw, found {:?}", 
                        prog.row, prog.token);
                    std::process::exit(super::COMPILE_ERROR);
                }
            }
        }
        None => {
            eprintln!("file is empty");
            std::process::exit(super::COMPILE_ERROR);
        }
    }
}
