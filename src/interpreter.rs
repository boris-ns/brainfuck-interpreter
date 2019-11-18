const MEMORY_CELL_SIZE: usize = 128;

pub struct Interpreter {
    memory_cells: [u32; MEMORY_CELL_SIZE],
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
        let mut loop_begin_index = 0;
        let mut i = 0;

        loop {
            if (i >= tokens.len()) {
                break;
            }
            
            match tokens[i] {
                // Shift pointer to the right
                '>' => { self.pointer += 1; }

                // Shift pointer to the left
                '<' => { self.pointer -= 1; }

                // Increment cell where pointer points at
                '+' => { self.memory_cells[self.pointer] += 1; }

                // Decrement cell where pointer points at
                '-' => { self.memory_cells[self.pointer] -= 1; }

                // Start loop
                '[' => {}

                // End loop
                ']' => {}

                // Print ASCII
                // @TODO: Find a better way to print
                '.' => { print!("{}", ((self.memory_cells[self.pointer]) as u8) as char); }

                // Get ASCII
                ',' => {}

                // Ignore everything else
                _ => {}
            }
        
            i += 1;
        }
    }
}