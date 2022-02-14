use std::env;
use std::process::Command;

fn bin_checker(bin: String) -> Result<(), ()> {
    let output = Command::new(bin).output();
    if output.is_ok() {
        return Ok(());
    } else {
        return Err(()) 
    };
}

fn help() {
    println!("Usage: bin_check {{binary_name}}")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        // no arguments passed
        1 => {
            println!("I need the binary name!");
        },
        2 => {
            let bin = String::from(&args[1]);
            match bin_checker(bin.clone()) {
                Ok(_) => {
                    println!("Binary: {} ✅", bin);
                    std::process::exit(0)
                },
                Err(_) => {
                    eprintln!("Binary: {} ❌", bin);
                    std::process::exit(1)
                }
            }
        },
        _ => {
            help();
        }
    };
}
