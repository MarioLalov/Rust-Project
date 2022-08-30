use std::fs::File;
use std::io::prelude::*;

pub struct CTranslator {
    command: Vec<char>,
    command_pos: usize,

    file: File,
}

impl CTranslator {
    pub fn new(file_name: &str) -> CTranslator {
        CTranslator {
            command: Vec::new(),
            command_pos: 0,

            file: File::create(file_name).unwrap(),
        }
    }

    fn count_symblol(&mut self, symbl: char) -> usize {
        let mut cnt = 1;
        self.command_pos += 1;

        loop {
            let ch = self.command[self.command_pos];

            if ch != symbl {
                // move back to previous symbol
                self.command_pos -= 1;

                return cnt;
            } else if self.command_pos >= self.command.len()-1 {
                // last symbol reached
                return cnt+1;
            }

            cnt += 1;
            self.command_pos += 1;
        }
    }

    fn act_on_right_move(&mut self) -> Result<(), std::io::Error> {
        let move_count = self.count_symblol('>');

        let ptr_increment = String::from("ptr += ") + &move_count.to_string() + ";\n";
        self.file.write_all(ptr_increment.as_bytes())
    }

    fn act_on_left_move(&mut self) -> Result<(), std::io::Error> {
        let move_count = self.count_symblol('<');

        let ptr_decrement = String::from("ptr -= ") + &move_count.to_string() + ";\n";
        self.file.write_all(ptr_decrement.as_bytes())
    }

    fn act_on_increment(&mut self) -> Result<(), std::io::Error> {
        let incr_count = self.count_symblol('+');

        let ptr_increment = String::from("*ptr += ") + &incr_count.to_string() + ";\n";
        self.file.write_all(ptr_increment.as_bytes())
    }

    fn act_on_decrement(&mut self) -> Result<(), std::io::Error> {
        let decr_count = self.count_symblol('-');

        let ptr_increment = String::from("*ptr -= ") + &decr_count.to_string() + ";\n";
        self.file.write_all(ptr_increment.as_bytes())
    }

    fn act_on_lbracket(&mut self) -> Result<(), std::io::Error> {
        let while_str = String::from("while (*ptr)\n{\n");
        self.file.write_all(while_str.as_bytes())
    }

    fn act_on_rbracket(&mut self) -> Result<(), std::io::Error> {
        self.file.write_all(String::from("}\n").as_bytes())
    }

    fn act_on_input(&mut self) -> Result<(), std::io::Error> {
        self.file.write_all(String::from("*ptr = getchar();\n").as_bytes())
    }

    fn act_on_print(&mut self) -> Result<(), std::io::Error> {
        self.file.write_all(String::from("putchar(*ptr);\n").as_bytes())
    }

    pub fn translate(&mut self, command: &str) {
        let begin_str = String::from("#include <stdio.h>\n\nint main()\n{\nchar array[30000] = {0};\nchar* ptr = array;\n");
        match self.file.write_all(begin_str.as_bytes()) {
            Ok(_) => (),
            Err(_) => return,
        };

        self.command = command.chars().collect();
        
        while self.command_pos < self.command.len() {
            let ch = self.command[self.command_pos];

            let result = match ch {
                                    '>' => self.act_on_right_move(),
                                    '<' => self.act_on_left_move(),
                                    '+' => self.act_on_increment(),
                                    '-' => self.act_on_decrement(),
                                    '[' => self.act_on_lbracket(),
                                    ']' => self.act_on_rbracket(),
                                    '.' => self.act_on_print(),
                                    ',' => self.act_on_input(),
                                    _   => Ok(()),
                                  };

            match result {
                Ok(_) => (),
                Err(_) => return,
            };

            self.command_pos += 1;
        }

        self.file.write_all("\nreturn 0;\n}".as_bytes());
    }
}