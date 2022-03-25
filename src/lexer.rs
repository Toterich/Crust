/**
 * The lexer takes as input the Crust source file to be compiled and outputs a stream of Tokens. This token stream in turn
 * is the input of the parser.
 *
 * Currently, the lexer implementation assumes the input file to contain only ASCII characters.
 */
use std::fs::read_to_string;

use super::errorhandler;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenClass {
    ERROR, // No valid token
    SINGLECHAR, // Combined class for all single-character tokens
    RETTYPE, // "->"
    FUNCTION,
    RETURN,
    INT32,
    INTLITERAL,
    IDENTIFIER,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub class: TokenClass,
    pub str: &'a str,

    // position inside file
    pub line: usize,
    pub pos: usize,
}

fn is_whitespace(c: char) -> bool
{
    match c
    {
        ' ' | '\t' | '\n' | '\r' => true,
        _ => false
    }
}

trait TokenChecker {
    fn check_token(&self, buffer: &str) -> usize;
}

struct SingleCharTokenChecker {}
impl TokenChecker for SingleCharTokenChecker {
    fn check_token(&self, buffer: &str) -> usize {
        let char = buffer.chars().next();
        match char {
            None => 0, // No chars in buffer
            Some(c) => match c {
                '!' | '&' | '|' | '^' | '~' | '?' |
                '(' | ')' | '{' | '}' | '[' | ']' |
                '*' | '+' | '-' | '/' | '<' | '>' | '=' |
                ',' | '.' | ';' | ':' => 1, // Valid single-char token
                _ => 0 // No valid single-char token
            }
        }
    }
}

struct FixedStringTokenChecker {
    str: String,
}
impl FixedStringTokenChecker {
    fn new<T: ToString>(str: T) -> FixedStringTokenChecker {
        FixedStringTokenChecker{str: str.to_string()}
    }
}
impl TokenChecker for FixedStringTokenChecker {
    /// Returns either self.str.len() if str is the first token in buffer, or 0 if it isn't
    fn check_token(&self, buffer: &str) -> usize {
        if buffer.starts_with(&self.str) {
            return self.str.len();
        }

        return 0;
    }
}

struct IdentifierTokenChecker {}
impl TokenChecker for IdentifierTokenChecker {
    fn check_token(&self, buffer: &str) -> usize
    {
        // Check each character of the buffer
        for (i, c) in buffer.chars().enumerate()
        {
            // On token end or EOF, we either matched or not
            if !(  (c >= 'a' && c <= 'z') // end match on non-matching character
                || (c >= 'A' && c <= 'Z')
                || (c == '_')
                || ((i != 0) && (c >= '0' && c <= '9'))) // Digits are not allowed as first character
            {
                return i;
            }
        }

        // Reached EOF and all chars matched
        return buffer.len();
    }
}

struct IntLiteralTokenChecker {}
impl TokenChecker for IntLiteralTokenChecker {
    fn check_token(&self, buffer: &str) -> usize
    {
        // Check each character of the buffer
        for (i, c) in buffer.chars().enumerate()
        {
            // On token end or EOF, we either matched or not
            if !(c >= '0' && c <= '9')    // end match on non-matching character
            {
                return i;
            }
        }

        // Reached EOF and all chars matched
        return buffer.len();
    }
}

pub struct Lexer {
    read_pos: usize,
    input_buffer: String,
}

struct TokenCandidate {
    class: TokenClass,
    length: usize
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer{read_pos: 0, input_buffer: "".to_string()}
    }

    pub fn load_input_from_file(&mut self, file_descriptor: &str) -> bool {
        let result = read_to_string(file_descriptor);
        // TODO[MAINT]: Handle errors in this function's caller instead
        match result {
            Ok(str) => {self.input_buffer = str; self.read_pos = 0; true},
            Err(e) => {errorhandler::file_read_error(file_descriptor, &e); false},
        }
    }

    // TODO[NICE]: Create an iterator instead?
    pub fn get_next_token(&mut self) -> Token {

        // Throw away all whitespace
        for c in self.input_buffer[self.read_pos..].chars()
        {
            if is_whitespace(c) {
                self.read_pos += 1;
            }
            else {
                break;
            }
        }

        let current_input_buffer = &self.input_buffer[self.read_pos..];

        let mut candidate = TokenCandidate{class: TokenClass::ERROR, length: 1};

        // Try to match the front of the buffer to the supported tokens
        // This respects maximal munch and token precedence. Tokens are ordered
        // in ascending precedence in the following

        // TODO[FEAT]: Add checks for all tokens of the Crust programming language
        // TODO[PERF]: Create constants for the TokenChecker instances
        // TODO[PERF]: This re-reads the input_buffer's characters for each Token. Maybe it would be better to only read each input
        //             char once and use an incremental check_char method on the TokenCheckers instead?

        candidate = Self::_check_token(current_input_buffer, TokenClass::IDENTIFIER, &IdentifierTokenChecker{}, candidate);
        candidate = Self::_check_token(current_input_buffer, TokenClass::INTLITERAL, &IntLiteralTokenChecker{}, candidate);
        candidate = Self::_check_token(current_input_buffer, TokenClass::SINGLECHAR, &SingleCharTokenChecker{}, candidate);
        candidate = Self::_check_token(current_input_buffer, TokenClass::RETTYPE, &FixedStringTokenChecker::new("->"), candidate);
        candidate = Self::_check_token(current_input_buffer, TokenClass::FUNCTION, &FixedStringTokenChecker::new("function"), candidate);
        candidate = Self::_check_token(current_input_buffer, TokenClass::INT32, &FixedStringTokenChecker::new("int32"), candidate);
        candidate = Self::_check_token(current_input_buffer, TokenClass::RETURN, &FixedStringTokenChecker::new("return"), candidate);

        let token =  Token{class: candidate.class,
                           str: &current_input_buffer[0..candidate.length],
                           line: 0, // TODO: Attach proper token position
                           pos: self.read_pos};

        // No token matched, so report an error
        if token.class == TokenClass::ERROR {
            errorhandler::token_error(&token);
        }

        // Advance the read buffer
        self.read_pos += candidate.length;

        return token;
    }

    fn _check_token(buffer: &str, token: TokenClass, checker: &dyn TokenChecker, prev_candidate: TokenCandidate) -> TokenCandidate{
        let length = checker.check_token(buffer);
        if length >= prev_candidate.length {
            return TokenCandidate{class: token, length: length};
        }
        return prev_candidate;
    }
}
