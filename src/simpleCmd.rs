// Simple Commands
// feld-cli
// -------------------------------------------------------- //

use crate::utils::*;

pub fn printHelp() {
    println!("Hello ! How to use feld :");
    println!("\n");
    println!("  version     Print feld version");
    println!("  -h          Show this message");
    println!("  hello       Print a greeting !");
    println!("  goodbye     Print a farewell");
    println!("  quote       Print a random quote");
}

pub fn printVersion(){
    println!("?")
}

pub fn printHello(){
    println!("Hello ! How are you ?")
}

pub fn printBye() {
    println!("See ya later . Take care !")
}

pub fn printQuote() {
    let quotes = [ 
        "Think well, in all you do .",
        "Continuos improvement .",
        "Maintain your identity . ",
    ];

    let result = getRandomIndex(quotes.len());

    println!("{}", quotes[result]);
}
