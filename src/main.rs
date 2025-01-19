// Main  
// feld-cli
// -------------------------------------------------------- //

#![allow(non_snake_case)]
use std::env;
use crate::simpleCmd::*;
use crate::strct::*;
//use crate::utils::*;
mod simpleCmd;
mod utils;
mod strct;

const SIMPLE_COMMANDS: [SimpleCommand; 5] = [
    SimpleCommand {
        name: "help",
        action: printHelp,
    },
    SimpleCommand {
        name: "version",
        action: printVersion,
    },
    SimpleCommand {
        name: "hello",
        action: printHello,
    },
    SimpleCommand {
        name: "goodbye",
        action: printBye,
    },
    SimpleCommand {
        name: "quote",
        action: printQuote,
    },
];

fn main() {
    let userInput: Vec<String> = env::args().collect();
    let inputLen = userInput.len();

    if inputLen < 2 {
        println!("Welcome . try -h for help . ");
        return;
    }

    processInputVec(userInput, inputLen);
}

fn processInputVec(input: Vec<String>, inputLen: usize) {
    match checkSingleInput(&input, inputLen) {
        Ok(true) => return,
        Err(msg) => { 
            println!("OoOps: {}", msg); 
            return;
        },
        Ok(false) => {},
    }

    if let Err(msg) = checkComplexInput(&input) {
        println!("OoOps: {}", msg);
        return;
    }
}

fn checkComplexInput(input: &[String]) -> Result<(), String> {
    println!("multiple CLI args: {:?}", input);
    return Ok(());
}

fn checkSingleInput(input: &[String], inputLen: usize) -> Result<bool, String> {
    let firstArg = input[1].as_str();

    if let Some(command) = SIMPLE_COMMANDS.iter().find(|cmd| cmd.name == firstArg) {
        if inputLen > 2 {
            return Err(format!("'{}' does not need arugments .", firstArg));
        }

        (command.action)();
        return Ok(true);
    }

    Ok(false)
}

