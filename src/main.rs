use std::env;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO[FEAT] Add proper argument parsing
    assert!(args.len() == 2);

    let mut lexer = lexer::Lexer::new();
    lexer.load_input_from_file(&args[1]);

    let token = lexer.get_next_token();
}
