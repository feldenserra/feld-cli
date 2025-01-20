// Simple Commands
// feld-cli
// -------------------------------------------------------- //
use crate::utils::*;
use colored::*;

pub struct SimpleCommand {
    pub name: &'static str,
    pub action: fn() -> (),
}

pub struct ComplexCommand {
    pub name: &'static[&'static str],
    pub action: fn(&'static[(&'static str, &'static[&'static str])], &[String], usize) -> (),
    pub params: &'static[(&'static str, &'static[&'static str])],
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
            ("check", &["-now"]),
            ("do", &["-now"]),
            ("myst", &["-ic"]),
        ],
    },
];

//~~ Notso
fn ProcessParams(params: &[(&str,&[&str])], input: &[String], usrIndex: usize) -> Result<(Option<String>,Option<String>), (Option<String>,Option<String>)> {
    let Some(userOption) = input.get(usrIndex) else {
        return Ok((None, None))
    };

    let Some(tuple) = params.iter().find(|t| t.0 == userOption) else {
        return Err((Some(format!("{}", userOption)), None))
    };

    let Some(subOpt) = input.get(usrIndex+1) else {
        return Ok((Some(format!("{}", userOption)), None))
    };

    let Some(_) = tuple.1.iter().find(|t| *t == subOpt) else {
        return Err((Some(format!("{}", userOption)), Some(format!("{}", subOpt))))
    };

    return Ok((Some(format!("{}", userOption)), Some(format!("{}", subOpt))))
}

fn handleSay(params: &[(&str,&[&str])], input: &[String], inputLen: usize) {
    if inputLen < 3 {
        println!("feld needs something to say .");
        println!("~~ try feld say -h");
        return;
    }

    let userInput = &input[2];

    if userInput == "-h"{
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

    match ProcessParams(params, input, 3) {
        Ok((None,None)) => {
            println!("feld says {}", input[2]);
        },
        Ok((Some(option), None)) => {
            formatSayText(&option, None, userInput);
        },
        Ok((Some(option), Some(subOpt))) => {
            formatSayText(&option, Some(&subOpt), userInput);
        },
        Err((Some(option), None)) => {
            println!("'say' doesnt know what '{}' is .", option);
        },
        Err((Some(option), Some(subOpt))) => {
            println!("'say {}' doesnt know what '{}' is .", option, subOpt);
        },
        _ => {
            println!("'say' got lost .");
        },
    }

}

fn formatSayText(option: &str, subOpt: Option<&str>, input:  &str) {
    let mut output = input.normal();

    match option {
        "loud" => {
            output = output.yellow();
        },
        "quiet" => {
            output = output.italic();
        },
        "myst" => {
            output = output.blue();
        },
        _ => {
            println!("'say' got lost ."); return; 
        },
    }

    let Some(subOption) = subOpt else {
        println!("feld says {}", output); return; 
    };

    match subOption {
        "-er" => {
            output = output.underline();
        },
        "-ic" => {
            output = output.blink();
        },
        _ => {
            println!("'say' got lost ."); return; 
        },
    }

    println!("feld says {}", output);
}

fn handleSlk(params: &[(&str,&[&str])], input: &[String], inputLen: usize) {
    if inputLen < 3 {
        println!("feld needs something to store .");
        println!("~~ try feld slk -h");
        return;
    }

    if input[2] == "-h"{
        println!("~~ slk -h");
        println!("  feld slk [1] [2] -[3], whatToDo, option, optMods");
        println!("  check (-now;)    checks all");
        println!("  do (-now;)       does all");
        println!("  myst (-ic;)      ~~");
        println!("\n");
        println!("~~ Examples");
        println!("feld slk check");
        println!("feld slk do -now");
        return;
    }

    match ProcessParams(params, input, 2) {
        Ok((None,None)) => {
            println!("slk_wip: standard, no params");
        },
        Ok((Some(option), None)) => {
            println!("slk_wip: opt-{}", option);
        },
        Ok((Some(option), Some(subOpt))) => {
            println!("slk_wip: opt-{},subOpt-{}", option, subOpt);
        },
        Err((Some(option), None)) => {
            println!("slk_wip: Error||| opt-{}", option);
        },
        Err((Some(option), Some(subOpt))) => {
            println!("slk_wip: Error||| opt-{}, subOpt-{}", option, subOpt);
        },
        _ => {
            println!("slk_wip:'say' got lost .");
        },
    }
}

//~~ Simple
fn printHelp() {
    println!("~~ Hello ! How to use feld :");
    println!("");
    println!("~~ Simple ~~");
    println!("  version     Print feld version");
    println!("  -h          Show this message");
    println!("  hello       Print a greeting !");
    println!("  goodbye     Print a farewell");
    println!("  quote       Print a random quote");
    println!("");
    println!("~~ Notso ~~");
    println!("use '-h' with any Notso command for help");
    println!("  say         says");
    println!("  slk         Store your Learned Knowledge");
}

fn printVersion(){
    println!("~~ feld-cli {} ~~", env!("CARGO_PKG_VERSION"));
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
}

fn printHello(){
    println!("Hello ! How are you ?")
}

fn printBye() {
    println!("See ya later . Take care !")
}

fn printQuote() {
    let quotes = [ 
        "Think well, in all you do .",
        "Continuos improvement .",
        "Maintain your identity . ",
    ];

    let result = getRandomIndex(quotes.len());

    println!("{}", quotes[result]);
}
