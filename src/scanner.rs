#[derive(Debug, Clone)]
pub enum Token {
    SRaw, SClass, SFn,
    Defun(String, usize), Endef,
    Defcl(String), Endcl,
    Pushi(i32), Pushf(f32), Pushv(usize),
    Pop(usize),
    Load(usize), Store(usize),
    Stores(usize, String),
    Alias(usize, usize),
    Add, Sub, Mul, Div, Rem,
    Call(String),
    Label(usize), Goto(usize)
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
                let pars = self.next_word().parse::<usize>().unwrap();
                Some(Inst {token: Token::Defun(name, pars), row: self.row})
            }
            "endef" => {
                Some(Inst {token: Token::Endef, row: self.row})
            }
            "load" => {
                let var = self.next_word().parse::<usize>().unwrap();
                Some(Inst {token: Token::Load(var), row: self.row})
            }
            "stores" => {
                let heap = self.next_word().parse::<usize>().unwrap();
                let _size = self.next_word().parse::<usize>().unwrap();
                let data = self.next_word();
                Some(Inst {token: Token::Stores(heap, data), row: self.row})
            }
            "call" => {
                let fun = self.next_word();
                Some(Inst {token: Token::Call(fun), row: self.row})
            }
            _ => None,
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
        self.update();
        return val;
    }
}
