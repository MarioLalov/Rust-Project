use std::io::{self, Write};

use crate::ctranslator::*;
use crate::interpreter::*;

fn get_input() -> String {
    let mut input = String::new();

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to get input!");

        let line_len = line.len();
        input.reserve(line_len);

        if line_len > 1 {
            // check for end string
            let end_str = &line[line_len-4..line_len-2]; // exclude enter

            if end_str == ":q" {
                input += &line[0..line_len-4];

                clear_terminal();

                return input;
            }
        }

        input += &line;
    }
}

fn wait_for_key() {
    println!("Press any key to continue.");

    let mut buff = String::new();
    std::io::stdin()
        .read_line(&mut buff)
        .expect("Failed to get input!");
}

fn interpret(print_tape: bool) {
    clear_terminal();
    println!("Write your code:");

    let mut interpreter = Interpreter::new(print_tape);
    interpreter.interpret(&get_input());

    wait_for_key();
}

fn translate() {
    clear_terminal();
    println!("Write your code:");
    let mut translator = CTranslator::new("test.c");
    translator.translate(&get_input());

    wait_for_key();
}

fn interpret_menu() {
    loop { 
        clear_terminal();

        println!("Choose an option:\n 1. Back\n 2. Interpret with tape\n 3. Interpret without tape\n");
        print!("Input: ");
        // flush to avoid delay in print
        io::stdout().flush().unwrap();

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to get input!");
        
        // exclude enter
        match &input[0..input.len()-2] {
            "1" => return,
            "2" => interpret(true),
            "3" => interpret(false),
            _ => continue,
        };
    }
}

pub fn start_menu() {
    loop {
        clear_terminal();
        
        println!("Choose an option:\n 1. Exit\n 2. Interpret\n 3. Translate to C\n");
        print!("Input: ");
        // flush to avoid delay in print
        io::stdout().flush().unwrap();

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to get input!");
        
        // exclude enter
        match &input[0..input.len()-2] {
            "1" => return,
            "2" => interpret_menu(),
            "3" => translate(),
            _ => continue,
        };
    }
}

