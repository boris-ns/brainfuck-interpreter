# Brainfuck interpreter

This is a simple interpreter for Brainfuck programming language. Besides regular interpreter this project has 
interactive interpreter.

## How to use

Download and install [Rust compiler](https://www.rust-lang.org/).  
Clone this repository with:  
```
git clone https://github.com/boris-ns/brainfuck-interpreter.git
cd brainfuck-interpreter
```  

There are two ways to run this project:  
- Interactive mode: ```cargo run```
- Regular mode: ```cargo run path_to_file```

Interactive interpreter contains two commands:
- ```reset``` - resets the state of interpreter (sets all memory cells to 0 and sets the pointer to the first memory cell)
- ```exit``` - exits the program