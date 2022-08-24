use crate::memory_tape::*;

use std::io;

use std::{thread, time::Duration};

pub struct Interpreter {
    tape: MemoryTape,

    opened_brackets: Vec<usize>,
    command: Vec<char>,
    command_pos: usize,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            tape: MemoryTape::new(),

            opened_brackets: Vec::new(),
            command: Vec::new(),
            command_pos: 0,
        }
    }

    fn wait() {
        thread::sleep(Duration::from_millis(1500));
    }
    fn clear() {
        std::process::Command::new("cmd")
        .args(&["/c", "cls"])
        .spawn()
        .expect("cls command failed to start")
        .wait()
        .expect("failed to wait");
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

        self.opened_brackets.push(self.tape.head_position);
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
        println!("Input: ");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to get input!");

        if !input.is_ascii() {
            return;
        }

        //try to get as a digit directly
        let result = input.parse::<u8>();
        match result {
            Ok(digit) => self.tape.set_cell_value(digit),
            Err(_) =>    self.tape.set_cell_value(input.as_bytes()[0]),
        }
    }

    pub fn interpret(&mut self, command: &str) {
        self.command = command.chars().collect();
        self.tape.print_tape_sniplet();

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
                            Interpreter::clear();
                            self.tape.print_tape_sniplet();

                            self.command_pos += 1;
                            
                            continue;
                        },

                _   =>    {
                            self.command_pos += 1;
                            continue;
                          },
            };

            Interpreter::wait();
            Interpreter::clear();
            self.tape.print_tape_sniplet();

            self.command_pos += 1;
        }
    }
}