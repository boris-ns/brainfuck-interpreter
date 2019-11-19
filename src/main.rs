mod interpreter;

use std::env;
use std::fs;

fn read_from_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Error while rading file");
}

fn main() {
    let cmd_args: Vec<String> = env::args().collect();
    let mut interpreter = interpreter::Interpreter::new();

    if cmd_args.len() == 1 {           // Start interactive interpreter
        interpreter.run_interactive();        
    } else if cmd_args.len() == 2 {    // Start regular interpreter
        let file_path = &cmd_args[1];
        let source_code: String = read_from_file(file_path);
        interpreter.run(source_code);
    } else {
        println!("Program takes 1 argument, path to the file you want to interpret.");
    }
}
