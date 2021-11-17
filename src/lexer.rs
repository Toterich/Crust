use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenClass {
    ERROR,
    LPARENTHESIS,
    INT,
    VOID,
    IDENTIFIER,
}

pub struct Token<'a> {
    pub class: TokenClass,
    pub str: &'a str,
}

trait TokenChecker {
    fn check_token(&self, buffer: &str) -> usize;
}

struct FixedStringTokenChecker {
    str: String,
}

fn is_whitespace(c: char) -> bool
{
    match c
    {
        ' ' | '\t' | '\n' | '\r' => true,
        _ => false
    }
}

impl TokenChecker for FixedStringTokenChecker {
    /// Returns either self.str.len() if str is the first token in buffer, or 0 if it isn't
    fn check_token(&self, buffer: &str) -> usize
    {
        let fixed_str_length = self.str.len();
        let buffer_len = buffer.len();

        if fixed_str_length > buffer_len {
            return 0;
        }

        if buffer.starts_with(&self.str) {
            if   fixed_str_length == buffer_len // End of buffer
              || is_whitespace(buffer.as_bytes()[fixed_str_length] as char) { // Next character is a string divider

                return fixed_str_length;
            }
        }

        return 0;
    }
}

struct IdentifierTokenChecker {

}

impl TokenChecker for IdentifierTokenChecker {
    fn check_token(&self, buffer: &str) -> usize
    {
        // Check each character of the buffer
        for (i, c) in buffer.chars().enumerate()
        {
            // On token end or EOF, we either matched or not
            if    is_whitespace(c)           // end match on whitespace
               || !(  (c >= 'a' && c <= 'z') // end match on non-matching character
                   || (c >= 'A' && c <= 'Z')
                   || (c >= '0' && c <= '9')
                   || (c == '_')
            )
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

macro_rules! fixed_string_token_checker {
    ($string:literal) => { FixedStringTokenChecker{str: $string.to_string()} };
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer{read_pos: 0, input_buffer: "".to_string()}
    }

    pub fn load_input_from_file(&mut self, file_descriptor: &str) -> bool {
        let result = read_to_string(file_descriptor);
        // TODO[MAINT]: Handle errors in this function's caller instead
        match result {
            Ok(str) => {self.input_buffer = str; true},
            Err(e) => {println!("Error reading file {}: {}", file_descriptor, e); false},
        }
    }

    // TODO[NICE]: Create an iterator instead?
    pub fn get_next_token(&mut self) -> Token {

        let current_input_buffer = &self.input_buffer[self.read_pos..];

        let mut candidate = (TokenClass::ERROR, 1);

        // Try to match the front of the buffer to the supported tokens
        // This respects maximal munch and token precedence. Tokens are ordered
        // in ascending precedence in the following if-cascade

        // TODO[FEAT]: Add checks for all tokens of the C programming language
        // TODO[PERF]: Create constants for the TokenChecker instances

        candidate = Self::_check_token(current_input_buffer, TokenClass::IDENTIFIER, &IdentifierTokenChecker{}, candidate);
        candidate = Self::_check_token(current_input_buffer, TokenClass::VOID, &fixed_string_token_checker!("void"), candidate);
        candidate = Self::_check_token(current_input_buffer, TokenClass::INT, &fixed_string_token_checker!("int"), candidate);
        candidate = Self::_check_token(current_input_buffer, TokenClass::LPARENTHESIS, &fixed_string_token_checker!("("), candidate);

        if candidate.0 == TokenClass::ERROR {
            // Token could not be parsed
            // TODO[MAINT]: Handle errors in this function's caller instead
            println!("Could not read a valid token at input position {}.", self.read_pos);
            std::process::exit(-1);
        }

        // Advance the read buffer
        self.read_pos += candidate.1;

        return Token{class: candidate.0, str: &current_input_buffer[0..candidate.1]};
    }

    fn _check_token(buffer: &str, token: TokenClass, checker: &dyn TokenChecker, prev_candidate: (TokenClass, usize)) -> (TokenClass, usize) {
        let length = checker.check_token(buffer);
        if length >= prev_candidate.1 {
            return (token, length);
        }
        return prev_candidate;
    }
}
