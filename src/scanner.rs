use super::mem_alloc::Type;

#[derive(Debug, Clone)]
pub enum Token {
    SRaw, SClass, SFn,
    Defun(String, Vec<Type>, Type), Endef,
    Defcl(String), Endcl,
    Pushi(i32), Pushf(f32), Pushv(usize), Pop,
    Load(usize), Store(usize),
    Stores(usize, String),
    Alias(usize, usize),
    Add, Sub, Mul, Div, Rem,
    Eq, Ne, Lt, Le, Gt, Ge,
    Call(String), Dup,
    Label(usize), Goto(usize), Branch(usize)
}

#[derive(Debug, Clone)]
pub struct Inst {
    pub token : Token,
    pub row   : usize
}

pub struct Scanner {
    data  : String,
    index : usize,
    row   : usize,
    curr  : Option<Inst>
}

const SEPS : [char; 3] = [' ', '\n', '\r'];

impl Scanner {
    pub fn from_string(input : String) -> Scanner {
        let mut s = Scanner {data: input,index: 0, row: 0, curr: None};
        s.update();
        return s;
    }

    fn next_word(&mut self) -> String {
        if self.data.len() == self.index {
            return String::new();
        }
        while SEPS.contains(& (self.data.chars().nth(self.index).unwrap())) {
                if self.data.chars().nth(self.index).unwrap() == '\n' {
                    self.row += 1;
                }
                self.index += 1;
                if self.data.len() == self.index {
                    return String::new();
                }
        }

        if self.data.len() == self.index {
            return String::new();
        }

        if self.data.chars().nth(self.index).unwrap() == ';' {
            while self.data.chars().nth(self.index).unwrap() != '\n' {
                self.index += 1;
            }
            return self.next_word();
        }

        let mut result = String::new();

        if self.data.chars().nth(self.index).unwrap() == '"' {
            self.index += 1;
            while self.data.chars().nth(self.index).unwrap() != '"' {
                result.push(self.data.chars().nth(self.index).unwrap());
                self.index += 1;
                if self.data.len() == self.index {
                    eprintln!("line {}: right quotation missing", self.row);
                    return String::new();
                }
            }
            self.index += 1;

            return result;
        } else {
            while ! SEPS.contains(& (self.data.chars().nth(self.index).unwrap())) {
                result.push(self.data.chars().nth(self.index).unwrap());
                self.index += 1;
            }
    
            return result;
        }
    }
    
    fn update(&mut self) {
        let nt = self.next_word();
        self.curr = match nt.as_str() {
            "" => None,
            ".raw" => Some(Inst {token: Token::SRaw, row: self.row}),
            ".class" => Some(Inst {token: Token::SClass, row: self.row}),
            ".function" => Some(Inst {token: Token::SFn, row: self.row}),
            "defun" => {
                let name = self.next_word();
                let pars : usize = self.next_word().parse().unwrap();
                let mut par_types : Vec<Type> = Vec::new();
                for _ in 0..pars {
                    par_types.push(Type::from_string(self.next_word()));
                }
                let ret_type = Type::from_string(self.next_word());
                Some(Inst {token: Token::Defun(name, par_types, ret_type), row: self.row})
            }
            "endef" => {
                Some(Inst {token: Token::Endef, row: self.row})
            }
            "pushi" => {
                let immediate : i32 = self.next_word().parse().unwrap();
                Some(Inst {token: Token::Pushi(immediate), row: self.row})
            }
            "pushf" => {
                let immediate : f32 = self.next_word().parse().unwrap();
                Some(Inst {token: Token::Pushf(immediate), row: self.row})
            }
            "load" => {
                let var : usize = self.next_word().parse().unwrap();
                Some(Inst {token: Token::Load(var), row: self.row})
            }
            "store" => {
                let var : usize = self.next_word().parse().unwrap();
                Some(Inst {token: Token::Store(var), row: self.row})
            }
            "stores" => {
                let heap : usize = self.next_word().parse().unwrap();
                let _size : usize = self.next_word().parse().unwrap();
                let data = self.next_word();
                Some(Inst {token: Token::Stores(heap, data), row: self.row})
            }
            "call" => {
                let fun = self.next_word();
                Some(Inst {token: Token::Call(fun), row: self.row})
            }
            "dup" => {
                Some(Inst {token: Token::Dup, row: self.row})
            }
            "label" => {
                let lbl : usize = self.next_word().parse().unwrap();
                Some(Inst {token: Token::Label(lbl), row: self.row})
            }
            "goto" => {
                let lbl : usize = self.next_word().parse().unwrap();
                Some(Inst {token: Token::Goto(lbl), row : self.row})
            }
            "branch" => {
                let lbl : usize = self.next_word().parse().unwrap();
                Some(Inst {token: Token::Branch(lbl), row : self.row})
            }
            "add" => Some(Inst {token: Token::Add, row : self.row}),
            "sub" => Some(Inst {token: Token::Sub, row : self.row}),
            "mul" => Some(Inst {token: Token::Mul, row : self.row}),
            "div" => Some(Inst {token: Token::Div, row : self.row}),
            "rem" => Some(Inst {token: Token::Rem, row : self.row}),
            "eq" => Some(Inst {token: Token::Eq, row : self.row}),
            "ne" => Some(Inst {token: Token::Ne, row : self.row}),
            "lt" => Some(Inst {token: Token::Lt, row : self.row}),
            "le" => Some(Inst {token: Token::Le, row : self.row}),
            "gt" => Some(Inst {token: Token::Gt, row : self.row}),
            "ge" => Some(Inst {token: Token::Ge, row : self.row}),
            _ => panic!{
                "(scanner) line {}: {} is not a valid instruction", self.row, nt
            },
        };
    }

    pub fn has_next(&mut self) -> bool {
        return match self.curr {
            Some(_) => true,
            None => false
        };
    }

    pub fn peek(&mut self) -> Option<Inst> {
        return self.curr.clone();
    }

    pub fn next(&mut self) -> Option<Inst> {
        let val = self.curr.clone();
        // println!("{:?}", val);
        self.update();
        return val;
    }
}
