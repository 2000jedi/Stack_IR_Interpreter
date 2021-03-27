use super::scanner;

#[derive(Debug)]
pub struct Exec {
    pub tokens : Vec<scanner::Token>,
}

#[derive(Debug)]
pub struct Program {
    pub class : Class,
    pub func : Fn,
}

#[derive(Debug)]
pub struct Class {
}

#[derive(Debug)]
pub struct DeFun {
    pub name : String,
    pub pars : usize,
    pub exec : Exec
}

#[derive(Debug)]
pub struct Fn {
    pub defs : Vec<DeFun>,
}


pub fn make_class(mut scan: scanner::Scanner) -> (Class, scanner::Scanner) {
    match scan.next() {
        Some(v) => {
            match v.token {
                scanner::Token::SClass => {
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

pub fn make_execs(mut scan: scanner::Scanner) -> (Exec, scanner::Scanner) {
    let mut execs = Exec{tokens: vec![]};
    loop {
        match scan.peek() {
            Some(v) => {
                match v.token {
                    scanner::Token::Endef => {
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

pub fn make_defun(mut scan: scanner::Scanner) -> 
        (Option<DeFun>, scanner::Scanner) {
    match scan.peek() {
        Some(v) => {
            match v.token {
                scanner::Token::Defun(name, pars) => {
                    scan.next();
                    let (exec, mut scan) = make_execs(scan);
                    match scan.next() {
                        Some(v) => {
                            match v.token {
                                scanner::Token::Endef => {}
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

pub fn make_fn(mut scan: scanner::Scanner) -> (Fn, scanner::Scanner) {
    match scan.next() {
        Some(v) => {
            match v.token {
                scanner::Token::SFn => {
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

pub fn make_ast(mut scan : scanner::Scanner) -> Program {
    match scan.next() {
        Some(prog) => {
            match prog.token {
                scanner::Token::SRaw => {
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
