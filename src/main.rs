mod interpreter;

use std::env;
use std::fs;

fn read_from_file(file_path: &str) -> Vec<char> {
    let source_code = fs::read_to_string(file_path).expect("Error while rading file");
    return source_code.chars().collect();
}

fn main() {
    let cmd_args: Vec<String> = env::args().collect();
    
    // TODO: Refactor this later. Better way of handling errors ?
    if cmd_args.len() != 2 {
        panic!("You must provide brainfuck source file");
    }
    
    let file_path = &cmd_args[1];
    let tokens: Vec<char> = read_from_file(file_path);

    let mut interpreter = interpreter::Interpreter::new();
    interpreter.run(tokens);
}
