use multilang::grammar::{Grammar, Rule};
use multilang::node::Context;
use multilang::parser::Parser as MLParser;

use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the grammar file
    grammar_path: PathBuf,

    /// Path to the code file
    code_path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let grammar_def = fs::read_to_string(&cli.grammar_path).expect(&format!(
        "Failed to read grammar file: {:?}",
        cli.grammar_path
    ));

    let input = fs::read_to_string(&cli.code_path)
        .expect(&format!("Failed to read code file: {:?}", cli.code_path));

    let grammar = Grammar::parse(&grammar_def);

    let parser = MLParser::new(&grammar, &input);
    match parser.parse(Rule::Program) {
        Ok(program_node) => {
            let mut ctx = Context::new();

            let result = program_node.run(&mut ctx);
            if let Err(e) = result {
                println!("Runtime Error: {}", e);
            }
        }
        Err(e) => {
            println!("Parsing Error: {}", e);
        }
    }
}
