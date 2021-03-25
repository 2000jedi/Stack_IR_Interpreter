/**
 * rvmi -- The Rust VM Interpreter.
 */

use std::fs;
use std::env;

pub mod scanner;

fn print_help() {
    println!("NAME");
    println!("     rvmi -- The Rust VM Interpreter");
    println!();
    println!("SYNOPSIS");
    println!("     rvmi [file]");
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let path = match args.get(1) {
        Some(t) => t,
        None => {
            print_help();
            return;
        }
    };
    let mut scanner = scanner::Scanner::from_string(fs::read_to_string(path)
        .unwrap());
    while scanner.has_next() {
        println!("{:?}", scanner.next());
    }
}
