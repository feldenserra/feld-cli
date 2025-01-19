// Simple Commands
// feld-cli
// -------------------------------------------------------- //
use crate::utils::*;

pub struct SimpleCommand {
    pub name: &'static str,
    pub action: fn() -> (),
}

pub const SIMPLE_COMMANDS: [SimpleCommand; 5] = [
    SimpleCommand {
        name: "-h",
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

pub struct ComplexCommand {
    pub name: &'static[&'static str],
    pub action: fn(&'static[(&'static str, &'static[&'static str])], &[String], usize) -> (),
    pub params: &'static[(&'static str, &'static[&'static str])],
}

pub const COMPLEX_COMMANDS: [ComplexCommand; 2] = [
    ComplexCommand {
        name: &["say", "echo"],
        action: handleSay,
        params: &[
            ("loud", &["-er"] ), 
            ("quiet", &["-er"]),
            ("myst", &["-ic"]),
        ],
    },
    ComplexCommand {
        name: &["slk", "git"],
        action: handleSlk,
        params: &[
            ("myst", &["ic"]),
        ],
    },
];

pub fn ProcessParams(params: &[(&str,&[&str])], input: &[String]) -> Result<(String,Option<String>), String> {
    match input.get(3) {
        Some(userOption)  => {
            if let Some(tuple) = params.iter().find(|t| t.0 == userOption) {
                match input.get(4) {
                    Some(subOpt)  => {
                        if let Some(subParam) = tuple.1.iter().find(|t| *t == subOpt) {
                            println!("option found: {}", userOption);
                            println!("subOption found: {}", subParam);
                            Ok((userOption.to_string(),Some(subParam.to_string())))
                        } else {
                            Ok((userOption.to_string(),None))
                        }
                        
                    },
                    None => {
                        println!("option found: {}", userOption);
                        Err(format!("Incorrect subOpt for {}", userOption))
                    },
                }
            } else {
                Err(format!("Incorrect option"))
            }
        }, 
        None => {
            Err(format!("None found matching"))
        },
    }
}

pub fn handleSay(params: &[(&str,&[&str])], input: &[String], inputLen: usize) {
    if inputLen < 3 {
        println!("feld needs something to say .");
        println!("~~ try feld say -h");
        return;
    }

    if input[2] == "-h"{
        println!("~~ say -h");
        println!("  feld say '[1]' [2] -[3], textToSay, option, optMods");
        println!("  loud (-er;)     extra emphasis");
        println!("  quiet (-er;)    less emphasis");
        println!("  myst (-ic;)     ~~");
        println!("\n");
        println!("~~ Examples");
        println!("feld say 'i love flowers'");
        println!("feld say 'for loop?' loud -er");
        return;
    }

    _ = ProcessParams(params, input);

    //println!("feld says {}", input[3]);
}

pub fn handleSlk(params: &[(&str,&[&str])], input: &[String], inputLen: usize) {
    if inputLen < 3 {
        println!("feld needs something to store .");
        println!("~~ try feld slk -h");
        return;
    }

    if input[2] == "-h"{
        println!("~~ slk -h");
        println!("  feld slk [1] [2] -[3], whatToDo, option, optMods");
        println!("  check (-org;)    checks all");
        println!("  do (-org;)       does all");
        println!("  myst (-ic;)      ~~");
        println!("\n");
        println!("~~ Examples");
        println!("feld slk check");
        println!("feld slk do -org");
        return;
    }

    println!("handleSlk in progress: {:?}", input);
}

pub fn printHelp() {
    println!("Hello ! How to use feld :");

    println!("\n");
    println!("~~ Simple ~~");
    println!("  version     Print feld version");
    println!("  -h          Show this message");
    println!("  hello       Print a greeting !");
    println!("  goodbye     Print a farewell");
    println!("  quote       Print a random quote");

    println!("\n");
    println!("~~ Not So ~~");
    println!("  say         says");
    println!("  slk         Store your Learned Knowledge");
}

pub fn printVersion(){
    println!("~~ feld-cli {} ~~", env!("CARGO_PKG_VERSION"));
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
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
