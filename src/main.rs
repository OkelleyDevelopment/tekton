pub mod composer;
pub mod snippets;
pub mod utils;

use std::env;
use std::process;

use snippets::errors::SnippetError;
use utils::write_to_file;

use crate::composer::compose_snippets;

fn help() {
    println!("cargo run <snippet file convert> <output file name>");
    process::exit(1);
}

fn parse_config(args: &[String]) -> (&String, String) {
    if args.len() < 2 {
        help();
    }
    return (&args[1], args[2].to_string());
}

fn main() -> Result<(), SnippetError> {
    let args: Vec<String> = env::args().collect();
    let (fname, file_to_write): (&String, String) = parse_config(&args);
    let result = compose_snippets(fname);
    match result {
        Ok(r) => {
            write_to_file(file_to_write, r);
            Ok(())
        }
        Err(e) => {
            Err(e)
        }
    }
}
