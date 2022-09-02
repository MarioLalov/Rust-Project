use crate::memory_tape::*;

use std::io::{self, Write};
use std::{thread, time::Duration};

pub fn clear_terminal() {
    // try to clear windows terminal
    match std::process::Command::new("cmd")
            .args(&["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait() 
            {
                Ok(_) => (),
                // if wait failed try to clear linux terminal
                Err(_) => {
                            std::process::Command::new("clear")
                                .spawn()
                                .expect("clear command failed to start")
                                .wait()
                                .expect("faild to wait");
                          } 
            };
}

pub struct Interpreter {
    tape: MemoryTape,
    should_print_tape: bool,

    opened_brackets: Vec<usize>,
    command: Vec<char>,
    command_pos: usize,

    ouput: String,
}

impl Interpreter {
    pub fn new(should_print_tape: bool) -> Interpreter {
        Interpreter {
            tape: MemoryTape::new(),
            should_print_tape: should_print_tape,

            opened_brackets: Vec::new(),
            command: Vec::new(),
            command_pos: 0,

            ouput: String::new(),   
        }
    }

    fn wait(&self) {
        if self.should_print_tape {
            thread::sleep(Duration::from_millis(1500));
        }
    }
    
    fn clear(&self) {
        if self.should_print_tape {
            clear_terminal();
        }
    }

    fn move_to_next_rbracket(&mut self) {
        loop {
            if self.command_pos >= self.command.len()
            {
                return;
            }

            let ch = self.command[self.command_pos];
            if ch == ']' {
                return;
            }

            self.command_pos += 1;
        }
    }

    fn act_on_lbracket(&mut self) {
        if self.tape.get_current_value() == 0  {
            // move to next command
            self.opened_brackets.pop();
            self.move_to_next_rbracket();

            return;
        }

        self.opened_brackets.push(self.command_pos);
    }

    fn act_on_rbracket(&mut self) {
        if self.tape.get_current_value() == 0 {
            // move to next command
            self.opened_brackets.pop();

            return;
        }

        self.command_pos = *self.opened_brackets.last().unwrap();
    }

    fn get_input(&mut self) {
        println!();
        print!("Input: ");
        // flush to avoid delay in print
        io::stdout().flush().unwrap();

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to get input!");

        if !input.is_ascii() {
            return;
        }

        //try to get as a digit directly
        let result = input.trim().parse::<u8>();
        match result {
            Ok(digit) => self.tape.set_cell_value(digit),
            Err(_) =>    self.tape.set_cell_value(input.as_bytes()[0]),
        }
    }

    fn print_command(&self, print_with_pos: bool) {
        if !self.should_print_tape {
            return;
        }

        let current_command = if print_with_pos {
                                String::from_iter(&self.command[0..self.command_pos])   +
                                "{" + &String::from(self.command[self.command_pos]) + "}" +
                                &String::from_iter(&self.command[self.command_pos+1..])
                              }else {
                                String::from_iter(&self.command)
                              };

        println!();
        println!("{}", current_command);
    }

    fn print_tape(&self) {
        if !self.should_print_tape {
            return;
        }

        self.tape.print_tape_sniplet();
        if !self.ouput.is_empty() {
            println!("Output: {}", self.ouput);
        }
    }

    pub fn interpret(&mut self, command: &str) {
        self.command = command.chars().collect();
        
        self.print_tape();
        self.print_command(false);

        while self.command_pos < self.command.len() {
            let ch = self.command[self.command_pos];

            match ch {
                '>' => self.tape.move_right(),
                '<' => self.tape.move_left(),
                '+' => self.tape.increment(),
                '-' => self.tape.decrement(),

                '[' => self.act_on_lbracket(),
                ']' => self.act_on_rbracket(),

                ',' => {
                            self.get_input();
                            self.clear();
                            self.print_tape();

                            self.command_pos += 1;
                            
                            continue;
                        },

                '.' =>  self.ouput.push(self.tape.get_current_value() as char),

                _   =>    {
                            self.command_pos += 1;
                            continue;
                          },
            };

            self.wait();
            self.clear();
            self.print_tape();
            self.print_command(true);

            self.command_pos += 1;
        }

        clear_terminal();
        println!("Result:\n");
        self.tape.print_tape_sniplet();
        println!("Output: {}", self.ouput);
    }
}