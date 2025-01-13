#![allow(non_snake_case)]
use std::env;
use std::time::{ SystemTime, UNIX_EPOCH };

fn main() {

    // get input
    let userInput: Vec<String> = env::args().collect();

    match userInput.get(1) 
    {
        Some(argument) => {
            let input = argument.as_str();
            processInput(input);
        }

        None => println!("Welcome . try -h for help . "),
    }
}

fn processInput(input: &str)
{
    match input
    {
        "hello" => println!("hello ! how are you ?"),
        "-h" | "--help" => printHelp(),
        "goodbye" => println!("See ya later . Take care !"),
        "quote" => printQuote(),

        // Default
        _ => println!("no commands : try -h for help ."),
    }
}

fn printQuote()
{
    let quotes = [ 
        "Think well, in all you do .",
        "Continuos Improvement ."
    ];

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() % 1000;

    let index = (now % quotes.len() as u128) as usize;
    let result = quotes[index];

    println!("{}", result);
}

fn printHelp()
{
    println!("Hello ! How to use feld :");
    println!("\n");
    println!("  hello       Print a greeting !");
    println!("  -h          Show this message");
    println!("  goodbye     Print a farewell");
    println!("  quote       Print a random quote");
}


