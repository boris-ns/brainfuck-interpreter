use std::collections::LinkedList;

// TODO: Maybe we can add this to be a parameter from command line
const MEMORY_CELL_SIZE: usize = 30000;

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

    pub fn run(&mut self, tokens: Vec<char>) {
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
                ',' => {}

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
}