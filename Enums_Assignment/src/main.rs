use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),               
    Display(String),            
    Create(String, String),     
    Remove(String),             
    Pwd,                        
}

fn main() {
    println!("\nWelcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        print!("\nEnter your choice (0-5): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("\nFailed to read input");
        let choice = choice.trim();

        let operation = match choice {
            "1" => {
                let dir = prompt("\nEnter directory path: ");
                Some(FileOperation::List(dir))
            }
            "2" => {
                let file = prompt("\nEnter file path: ");
                Some(FileOperation::Display(file))
            }
            "3" => {
                let file = prompt("\nEnter file path: ");
                let content = prompt("Enter content: ");
                Some(FileOperation::Create(file, content))
            }
            "4" => {
                let file = prompt("\nEnter file path: ");
                Some(FileOperation::Remove(file))
            }
            "5" => {
                println!("\nCurrent working directory: ");
                Some(FileOperation::Pwd)
            }
            "0" => {
                println!("\nGoodbye!\n");
                break;
            }
            _ => {
                println!("\nInvalid option. Please enter a number between 0 and 5.");
                None
            }
        };

        if let Some(op) = operation {
            perform_operation(op);
        }
    }
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(directory_path) => {
            let status = Command::new("ls")
                .arg(&directory_path)
                .status();

            match status {
                Ok(s) if s.success() => {}
                _ => eprintln!("Failed to execute ls"),
            }
        }

        FileOperation::Display(file_path) => {
            let status = Command::new("cat")
                .arg(&file_path)
                .status();

            match status {
                Ok(s) if s.success() => {}
                _ => eprintln!("Failed to execute cat"),
            }
        }

        FileOperation::Create(file_path, content) => {
            let command = format!("echo '{}' > {}", content, file_path);

            let status = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("File '{}' created successfully.", file_path);
                }
                _ => eprintln!("Failed to create file"),
            }
        }

        FileOperation::Remove(file_path) => {
            let status = Command::new("rm")
                .arg(&file_path)
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("File '{}' removed successfully.", file_path);
                }
                _ => eprintln!("Failed to remove file"),
            }
        }

        FileOperation::Pwd => {
            let status = Command::new("pwd")
                .status();

            match status {
                Ok(s) if s.success() => {}
                _ => eprintln!("Failed to execute pwd"),
            }
        }
    }
}