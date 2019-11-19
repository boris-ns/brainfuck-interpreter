use std::io::{self, Write};
use std::collections::LinkedList;

const MEMORY_CELL_SIZE: usize = 30000;

fn get_input_string() -> String {
    let mut buffer = String::new();
    buffer.clear();
    io::stdout().flush();
    io::stdin().read_line(&mut buffer).expect("Failed to read line from console");

    return String::from(buffer.trim());
}

fn get_input() -> u8 {
    println!("");
    let mut buffer = String::new();
    buffer.clear();
    io::stdout().flush();
    io::stdin().read_line(&mut buffer).expect("Failed to read line from console");

    return buffer.trim().parse::<u8>().unwrap();
}

pub struct Interpreter {
    memory_cells: [i8; MEMORY_CELL_SIZE],
    pointer: usize
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            memory_cells: [0; MEMORY_CELL_SIZE],
            pointer: 0
        }
    }

    pub fn run_interactive(&mut self) {
        loop {
            print!("# ");
            let command: String = get_input_string();

            match command.as_str() {
                "reset" => { self.reset(); }
                "exit"  => { return; }
                _       => { 
                    self.run(command); 
                    println!("");
                }
            };
        }
    }

    pub fn run(&mut self, source_code: String) {
        let tokens = source_code.chars().collect();
        self.interpret(tokens);
    }

    fn interpret(&mut self, tokens: Vec<char>) {
        let mut i = 0;
        let mut loop_stack: LinkedList<usize> = LinkedList::new();

        loop {
            if i >= tokens.len() {
                break;
            }

            match tokens[i] {
                // Shift pointer to the right
                '>' => { self.inc_pointer(); }

                // Shift pointer to the left
                '<' => { self.dec_pointer(); }

                // Increment cell where pointer points at
                '+' => { self.memory_cells[self.pointer] += 1; }

                // Decrement cell where pointer points at
                '-' => { self.memory_cells[self.pointer] -= 1; }

                // Start loop
                '[' => { loop_stack.push_front(i); }

                // End loop
                ']' => {
                    if self.memory_cells[self.pointer] == 0 {
                        loop_stack.pop_front();
                    } else {
                        i = *loop_stack.front().unwrap();
                    }
                }

                // Print ASCII
                '.' => { print!("{}", (self.memory_cells[self.pointer] as u8) as char); }

                // Get ASCII
                ',' => { self.write_to_cell(); }

                // Ignore everything else
                _ => {}
            }
            
            i += 1;
        }
    }

    fn inc_pointer(&mut self) {
        if self.pointer != MEMORY_CELL_SIZE {
            self.pointer += 1;
        }
    }

    fn dec_pointer(&mut self) {
        if self.pointer != 0 {
            self.pointer -= 1;
        }
    }

    fn write_to_cell(&mut self) {
        let value: u8 = get_input();
        self.memory_cells[self.pointer] = value as i8;
    }

    fn reset(&mut self) {
        self.memory_cells = [0; MEMORY_CELL_SIZE];
        self.pointer = 0;
    }
}