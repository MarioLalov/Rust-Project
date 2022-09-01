
use std::slice::Iter;
use std::fs::File;
use std::io::prelude::*;

pub fn parse<'a>(rows: &Vec<String>, whiles_number: usize, file: &mut File) -> Result<(), &'a str>{
    let mut iter = rows.iter();

    let mut functions : Vec<Vec<String>> = Vec::new();
    functions.resize(whiles_number, Vec::new());

    let mut fun_number = 0;
    divide_into_function(&mut iter, &mut fun_number, 0, &mut functions);

    write_to_file(file, &functions)
}

fn write_to_file<'a>(file: &mut File, functions: &Vec<Vec<String>>) -> Result<(), &'a str>{
    // write includes 
    let include_str = String::from("#include <stdio.h>\n\n");
    match file.write_all(include_str.as_bytes()) {
        Ok(_) => (),
        Err(_) => return Err("Couldn't write to file."),
    };

    // write declaration
    for i in 1..functions.len() {
        let declaration = String::from("void fun") + &i.to_string() + "();\n";
        match file.write_all(declaration.as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Couldn't write to file."),
        };
    }

    // write main
    let declaration = String::from("\nchar array[30000] = {0};\nchar* ptr = array;\nint main()\n{\n");
    match file.write_all(declaration.as_bytes()) {
        Ok(_) => (),
        Err(_) => return Err("Couldn't write to file."),
    };

    for row in &functions[0] {
        match file.write_all((String::from(row) + "\n").as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Couldn't write to file."),
        };
    }

    match file.write_all("\nreturn 0;\n}\n\n".as_bytes()) {
        Ok(_) => (),
        Err(_) => return Err("Couldn't write to file."),
    };

    // write definitions
    for fun_id in 1..functions.len() {
        let declaration = String::from("void fun") + &fun_id.to_string() + "()\n{\n";
        match file.write_all(declaration.as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Couldn't write to file."),
        };

        for row in &functions[fun_id] {
            match file.write_all((String::from(row) + "\n").as_bytes()) {
                Ok(_) => (),
                Err(_) => return Err("Couldn't write to file."),
            };
        }

        match file.write_all("}\n\n".as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Couldn't write to file."),
        };
    }

    Ok(())
}

fn divide_into_function(rows_iter: &mut Iter<String>,
                        next_fun_num: &mut usize,
                        current_fun_num: usize, 
                        functions: &mut Vec<Vec<String>>) {

    loop {
        let row = match rows_iter.next() {
                                            Some(row) => row,
                                            None => "",
                                        };

        if row == "" {
            // code end
            return;
        } else if row == "}" {
            // end of function
            functions[current_fun_num].push(String::from(row));

            return;
        } else if row == "while (*ptr)"{
            // add current functin
            *next_fun_num += 1;
            functions[current_fun_num].push(String::from("fun") + &(next_fun_num).to_string() + "();");
            // add first two rows of next function
            functions[*next_fun_num].push(String::from(row));

            let new_row = match rows_iter.next() {
                Some(row) => row,
                None => return,
            };
            functions[*next_fun_num].push(String::from(new_row));

            divide_into_function(rows_iter, next_fun_num, *next_fun_num, functions);
        } else {
            // standard add
            functions[current_fun_num].push(String::from(row));
        }
    }
}

#[test]
fn no_whiles() {
    let rows: Vec<String> = vec![String::from("ptr++;"), String::from("ptr++;"), String::from("ptr++;")];


}