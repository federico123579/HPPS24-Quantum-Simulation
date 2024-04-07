use std::path::PathBuf;

use clap::Parser as CParser;
use parser::Parser;

use crate::parser::parse_program;

mod error;
mod parser;
mod tokens;

#[derive(Debug, Clone, CParser)]
struct Cli {
    input: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let input_txt = std::fs::read_to_string(args.input).unwrap();

    let lex = Parser::new(&input_txt);
    let gates = parse_program(lex).unwrap();
    dbg!(gates);
}
