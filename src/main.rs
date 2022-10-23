mod case;

use anyhow::Result;
use case::Case;
use clap::Parser;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[arg(value_enum, help = "Case to convert to")]
    case: Case,

    #[arg(help = "String to convert; if empty, reads from stdin")]
    input: Option<String>,

    #[arg(
        short,
        long,
        help = "Case to convert from, if best-guess isn't working"
    )]
    from: Option<Case>,

    #[arg(long, help = "Debug mode")]
    debug: bool,
}

fn main() -> Result<()> {
    let cli = CLI::parse();

    let lines: Vec<String> = match cli.input {
        Some(input) => input.lines().map(|s| s.to_string()).collect::<Vec<_>>(),
        None => io::stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<_>>(),
    };

    for line in lines {
        let from = cli.from.to_owned().unwrap_or_else(|| Case::guess(&line));
        if cli.debug {
            println!("from: {:?}", from);
        }

        let tokens = from.tokenize(&line);
        if cli.debug {
            println!("tokens: {:?}", tokens);
        }

        let output = cli.case.join(tokens);
        println!("{}", output);
    }

    Ok(())
}
