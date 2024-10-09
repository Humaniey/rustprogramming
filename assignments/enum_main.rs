use std::fs;
use std::io::{self, Write};
use std::path::Path;

enum FileOperation {
    Create(String),
    Rename(String, String),
    Write(String),
}

impl FileOperation {
    get_user_input() -> String {
    let mut buffer: String = String::new();
    io::stdin().read_line(buf: &mut buffer).unwrap();
    let buffer: &str = buffer.trim();
    
    }
}

impl Write {
    
}
    

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::Create(filename) => {
    
            // TODO: Implement file creation logic
            println!("File '{}' created successfully.", filename);
        }
        FileOperation::Rename(old_name, new_name) => {
            // TODO: Implement file renaming logic
            
            println!("File renamed from '{}' to '{}' successfully.", old_name, new_name);
        }
        FileOperation::Write(text) => {
        
        }
    }
}

fn main() {
    fir 
    println!("Choose an operation:");
    println!("1. Create a new file");
    println!("2. Rename an existing file");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            println!("Type a name of the file you want to create")
            let new_file: String = FileOperation::get_user_input();
            perform_operation(FileOperation::Create(new_file));
            // TODO: Prompt for new filename and call perform_operation
        }
        "2" => {
            // TODO: Prompt for old and new filenames and call perform_operation
        }
        
        "3" => {
            
        }
            
        _ => println!("Invalid choice"),
    }
}