// Main  
// feld-cli
// -------------------------------------------------------- //

#![allow(non_snake_case)]
use std::env;
use crate::commands:: { COMPLEX_COMMANDS, SIMPLE_COMMANDS };
//use crate::utils::*;
mod commands;
mod utils;

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

    match checkComplexInput(&input, inputLen) {
        Ok(()) => return,
        Err(msg) => {
            println!("OoOps: {}", msg);
            println!("gh/feldenserra/feld-cli");
            return;
        },
    }
}

fn checkComplexInput(input: &[String], inputLen: usize) -> Result<(), String> {
    if let Some(command) = COMPLEX_COMMANDS.iter().find(|&cmd| cmd.name.contains(&input[1].as_str())) {
        (command.action)(command.params, input, inputLen);
        return Ok(());
    }

    return Err(format!("'{}' does not exist , should it ?", input[1].as_str()));
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

