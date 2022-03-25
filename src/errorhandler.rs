use std::io::Error as IoError;

use super::lexer::Token;

pub fn token_error(token: &Token) {
    println!("Token Error at line {}, pos {}: Can't parse token \'{:?}\'", token.line, token.pos, token.str);
    finish(-1);
}

pub fn file_read_error(filename: &str, error: &IoError) {
    println!("Error reading file {:?}: {}", filename, error);
    finish(-2);
}

fn finish(exit_code: i32) {
    std::process::exit(exit_code);
}
