use std::fs::File;

use crate::code_parser::*;

pub struct CTranslator {
    command: Vec<char>,
    command_pos: usize,
    whiles_count: usize,

    file: File,
    code_lines: Vec<String>,
}

fn get_arithmetic_str(operation: &str, number: usize) -> String{
    if number == 1 {
        String::from(operation) + operation
    } else {
        String::from(" ") + operation + "= " + &number.to_string()
    }
}

impl CTranslator {
    pub fn new(file_name: &str) -> Result<CTranslator, &str> {
        let ctranslator = CTranslator {
                            command: Vec::new(),
                            command_pos: 0,

                            whiles_count: 0,
                            code_lines: Vec::new(),

                            file: match File::create(file_name) {
                                Ok(file) => file,
                                Err(_) => return Err("Invalid file name.")
                            },
                        };

        return Ok(ctranslator);
    }

    fn count_symblol(&mut self, symbl: char) -> usize {
        let mut cnt = 1;
        self.command_pos += 1;

        loop {
            if self.command_pos > self.command.len()-1 
            {
                // last symbol reached
                return cnt;
            }

            let ch = self.command[self.command_pos];

            if ch != symbl {
                // move back to previous symbol
                self.command_pos -= 1;

                return cnt;
            }

            cnt += 1;
            self.command_pos += 1;
        }
    }

    fn act_on_right_move(&mut self) {
        let move_count = self.count_symblol('>');

        let ptr_increment = String::from("ptr") + &get_arithmetic_str("+", move_count) + ";";
        self.code_lines.push(ptr_increment);
    }

    fn act_on_left_move(&mut self) {
        let move_count = self.count_symblol('<');

        let ptr_decrement = String::from("ptr") + &get_arithmetic_str("-", move_count) + ";";
        self.code_lines.push(ptr_decrement);
    }

    fn act_on_increment(&mut self) {
        let incr_count = self.count_symblol('+');

        let ptr_increment = String::from("*ptr") + &get_arithmetic_str("+", incr_count) + ";";
        self.code_lines.push(ptr_increment);
    }

    fn act_on_decrement(&mut self) {
        let decr_count = self.count_symblol('-');

        let ptr_decrement = String::from("*ptr") + &get_arithmetic_str("-", decr_count) + ";";
        self.code_lines.push(ptr_decrement);
    }

    fn act_on_lbracket(&mut self) {
        let while_str = String::from("while (*ptr)");
        self.code_lines.push(while_str);
        self.code_lines.push(String::from("{"));
        self.whiles_count += 1;
    }

    fn act_on_rbracket(&mut self) {
        self.code_lines.push(String::from("}"));
    }

    fn act_on_input(&mut self) {
        self.code_lines.push(String::from("*ptr = getchar();"));
    }

    fn act_on_print(&mut self) {
        self.code_lines.push(String::from("putchar(*ptr);"));
        
    }

    pub fn translate<'a>(&mut self, command: &str) -> Result<(), &'a str> {
        self.command = command.chars().collect();
        
        self.command_pos = 0;
        while self.command_pos < self.command.len() {
            let ch = self.command[self.command_pos];

            match ch {
                '>' => self.act_on_right_move(),
                '<' => self.act_on_left_move(),
                '+' => self.act_on_increment(),
                '-' => self.act_on_decrement(),
                '[' => self.act_on_lbracket(),
                ']' => self.act_on_rbracket(),
                '.' => self.act_on_print(),
                ',' => self.act_on_input(),
                _   => (),
                };

            self.command_pos += 1;
        }

        parse(&self.code_lines, self.whiles_count + 1, &mut self.file)
    }
}

#[test]
fn increment_with_one() {
    let result = get_arithmetic_str("+", 1);
    assert_eq!(result, "++");
}

#[test]
fn increment() {
    let result = get_arithmetic_str("+", 7);
    assert_eq!(result, " += 7");
}

#[test]
fn decrement_with_one() {
    let result = get_arithmetic_str("-", 1);
    assert_eq!(result, "--");
}

#[test]
fn decrement() {
    let result = get_arithmetic_str("-", 7);
    assert_eq!(result, " -= 7");
}