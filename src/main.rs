#![allow(non_snake_case)]
use std::env;

fn main() {

    // get input
    
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(command) => {
            match command.as_str() {
                "hello" => println!("hello ! how are you ?"),
                "-h" | "--help" => print_Help(),
                _ => println!("no commands : try -h for help ."),
            }
        }
        None => println!("Welcome . try -h for help . "),
    }
}

fn print_Help()
{
    println!("Hello ! How to use feld :");
    println!("\n");
    println!("  hello       Print a greeting !");
    println!("  -h          Show this message");
    println!("  goodbye     Print a farewell");
    println!("  quote       Print a random quote");
}


