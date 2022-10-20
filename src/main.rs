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

    let input = cli.input.unwrap_or_else(|| {
        io::stdin()
            .lock()
            .lines()
            .next()
            .expect("failed to read from stdin")
            .expect("failed to read from stdin")
    });

    let from = cli.from.unwrap_or(Case::guess(&input));
    if cli.debug {
        println!("from: {:?}", from);
    }

    let tokens = from.tokenize(&input);
    if cli.debug {
        println!("tokens: {:?}", tokens);
    }

    let output = cli.case.join(tokens);

    println!("{}", output);

    Ok(())
}
