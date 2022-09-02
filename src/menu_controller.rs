use std::io::{self, Write};
use std::fs;

use crate::ctranslator::*;
use crate::interpreter::*;

enum InputType {
    File,
    Terminal,
}

// inputs
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

fn get_input_from_file() -> String {
    let mut file_name = String::new();
    println!();
    print!("Enter input file path: ");
    // flush to avoid delay in print
    io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to get input!");

    let  input = match fs::read_to_string(file_name.trim()) {
        Ok(text) => text,
        Err(_) => String::new(),
    };

    input
}

fn wait_for_key() {
    println!("Press enter to continue.");

    let mut buff = String::new();
    std::io::stdin()
        .read_line(&mut buff)
        .expect("Failed to get input!");
}

fn prepare_for_code_input() {
    clear_terminal();
    println!("Write your code:");
}

fn prepare_for_input_file() {
    clear_terminal();
}

fn interpret(print_tape: bool, prepare_fun: &dyn Fn()->(), input_fun: &dyn Fn()->String) {
    prepare_fun();

    let mut interpreter = Interpreter::new(print_tape);
    interpreter.interpret(&input_fun());

    wait_for_key();
}

fn create_translator() -> CTranslator {
    loop {
        println!();
        print!("Enter output file path: ");
        // flush to avoid delay in print
        io::stdout().flush().unwrap();

        let mut file_name = String::new();

        std::io::stdin()
            .read_line(&mut file_name)
            .expect("Failed to get input!");

        prepare_for_code_input();
        
        match CTranslator::new(&file_name.trim()) {
            Ok(translator) => return translator,
            Err(msg) => println!("{}",msg),
        };
    }
}

fn translate(prepare_fun: &dyn Fn()->(), input_fun: &dyn Fn()->String) {
    let mut translator = create_translator();

    prepare_fun();
    
    match translator.translate(&input_fun()) {
        Ok(_) => println!("C code successfully saved to {}", "file"),
        Err(msg) => println!("{}", msg),
    };

    wait_for_key();
}

fn file_or_input_menu() -> Option<InputType> {
    loop { 
        clear_terminal();

        println!("Choose an option:\n 1. Back\n 2. Use terminal as input\n 3. Use file as input\n");
        print!("Input: ");
        // flush to avoid delay in print
        io::stdout().flush().unwrap();

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to get input!");
        
        // exclude enter
        match &input[0..input.len()-2] {
            "1" => return None,
            "2" => return Some(InputType::Terminal),
            "3" => return Some(InputType::File),
            _ => continue,
        };
    }
}

fn interpret_menu(input_type: InputType) {
    let input_fun = match input_type {
                                        InputType::Terminal => get_input,
                                        InputType::File => get_input_from_file,
                                     };

    let prepare_fun = match input_type {
                                        InputType::Terminal => prepare_for_code_input,
                                        InputType::File => prepare_for_input_file,
                                        };

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
            "2" => interpret(true, &prepare_fun, &input_fun),
            "3" => interpret(false, &prepare_fun, &input_fun),
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
            "2" => {
                        match file_or_input_menu() {
                            Some(input_type) => interpret_menu(input_type),
                            None => continue,
                        };
                   }
                
            "3" => {
                        let input_type = match file_or_input_menu() {
                                                                        Some(input_type) => input_type,
                                                                        None => continue,
                                                                    };
                        let input_fun = match input_type {
                                                            InputType::Terminal => get_input,
                                                            InputType::File => get_input_from_file,
                                                         };
                    
                        let prepare_fun = match input_type {
                                                            InputType::Terminal => prepare_for_code_input,
                                                            InputType::File => prepare_for_input_file,
                                                           };

                        translate(&prepare_fun, &input_fun);
                   }   
            _ => continue,
        };
    }
}

