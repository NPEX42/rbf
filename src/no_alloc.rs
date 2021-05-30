#[cfg(not(feature = "no_std"))]
use std::fmt::{Display, Formatter, write};

#[cfg(feature = "no_std")]
use core::fmt::{Display, Formatter, write};


#[cfg(feature = "tinix")]  
    extern crate tinix_core;
#[cfg(feature = "tinix")] 
    use tinix_core as tinix;
#[cfg(feature = "tinix")] 
    use tinix::println;




#[derive(Debug, Clone, Copy)]
pub struct BrainFuckInterpreter<'a> {
    tape_ptr    : usize,
    program_ptr : usize,
    program     : &'a str,
    tape        : [u8;1024],
    jump_stack  : Stack<'a>,
}

impl<'a> BrainFuckInterpreter<'a> {
    pub fn new(program : &'a str) -> BrainFuckInterpreter {
        BrainFuckInterpreter {
            program_ptr : 0,
            jump_stack  : Stack::new(),
            program     : program,
            tape        : [0;1024],
            tape_ptr    : 0,
        }
    }

    pub fn run(mut self) {
        for _ in 0..self.program.len() {
            //println!("{}", self);
            match self.program.as_bytes()[self.program_ptr] {
                b'>' => self.tape_ptr += 1,
                b'<' => self.tape_ptr -= 1,
                b'+' => self.tape[self.tape_ptr] += 1,
                b'-' => self.tape[self.tape_ptr] -= 1,
                b'.' => print!("{}",self.tape[self.tape_ptr] as char),
                b'#' => print!("{}",self.tape[self.tape_ptr] as u8),
                b',' => {
                    let mut buffer = String::new();
                    std::io::stdin().read_line(&mut buffer).expect("Error Reading From STDIN..."); 
                    let first_char = buffer.as_bytes()[0];
                    self.tape[self.tape_ptr] = first_char;
                } 

                b'[' => {
                    if self.tape[self.tape_ptr] == 0 { //Skip to next ]
                        while self.program.as_bytes()[self.program_ptr] != b']' {
                            self.program_ptr += 1;
                        } 
                    } else {
                        self.jump_stack.push(self.program_ptr + 0);
                    }
                }

                b']' => {
                    if self.tape[self.tape_ptr] != 0 { //Jump to previous [
                        self.program_ptr = self.jump_stack.peek().unwrap_or(self.program_ptr);
                    } else {
                        let _ = self.jump_stack.pop();
                    }
                }
                _ => {}
            }
            self.program_ptr += 1;
        }
    }
}

impl Display for BrainFuckInterpreter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "Cell Ptr {}", self.tape_ptr);
        writeln!(f, "Current Cell {} ({})",self.tape[self.tape_ptr], self.tape[self.tape_ptr] as char);
        writeln!(f, "Program Counter: {}",self.program_ptr);
        writeln!(f, "Current Command: {}", self.program.as_bytes()[self.program_ptr - 1] as char);
        for idx in self.tape_ptr .. 256 {
            write!(f, "{},",self.tape[idx]);
        }
        writeln!(f, "\nStack: {}", self.jump_stack)
    }
}



#[derive(Debug, Clone, Copy)]
struct Stack<'a> {
    stack_ptr   : usize,
    stack       : &'a [usize; 256]
}

impl<'a> Stack<'a> {
    pub fn new() -> Stack<'a> {
        Stack {
            stack : &[0;256],
            stack_ptr : 0
        }
    }

    pub fn push(&mut self, data : usize) {
        let mut stack = *self.stack;
        stack[self.stack_ptr] = data;
        self.stack_ptr += 1;
    } 

    pub fn pop(&mut self) -> Option<usize> {
        if self.stack_ptr > 0 {
            self.stack_ptr -= 1;
            Some(self.stack[self.stack_ptr])
        } else {
            None
        }
    }

    pub fn peek(&mut self) -> Option<usize> {
        if self.stack_ptr > 0 {
            Some(self.stack[self.stack_ptr - 1])
        } else {
            None
        }
    }
}

impl Display for Stack<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for idx in 0..self.stack_ptr {
            write!(f,"[{}], ", self.stack[idx]);
        }
        writeln!(f, "")
    }
}