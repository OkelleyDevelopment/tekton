use std::path::Path;
use std::ffi::OsStr;

extern crate clap;
use clap::Parser;
use tekton::composer::compose_snippets;
use tekton::utils::write_to_file;
use tekton::errors::TektonError;

/// Entry point to the CLI App
fn main() -> Result<(), TektonError> {
    let cli = Cli::parse();
    let res: Result<(String, String), TektonError> = parse_config(cli);
    match res {
        Ok(names) => {
            let (fname, file_to_write) = names;
            let types: (Option<&str>, Option<&str>) = (get_extension_from_filename(&fname), get_extension_from_filename(&file_to_write));
            if types.0.is_some() && types.1.is_some() {
                //println!("Type 1: {:?}\nType 2: {:?}", types.0, types.1);
                let result = compose_snippets(&fname, (types.0.unwrap(), types.1.unwrap()));
                match result {
                    Ok(r) => {
                        if r.len() > 0 {
                            write_to_file(file_to_write, r);
                            Ok(())
                        } else {
                            Err(TektonError::Reason(String::from("Empty file will not be written")))
                        }
                    }
                    Err(e) => Err(e),
                }
                
            } else {
                Err(TektonError::Reason(String::from("Unable to extract file extension")))
            }

        }
        Err(e) => Err(TektonError::Reason(e.to_string())),
    }
}

/// Helper function to get the file extension being passed in.
fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The input snippet file name
    #[clap(value_parser)]
    input: Option<String>,

    /// The output snippet file name
    #[clap(value_parser)]
    output: Option<String>,
}

/// Parses the input and output file name arguments, returning an error if missing
fn parse_config(cli: Cli) -> Result<(String, String), TektonError> {
    let mut payload = (String::new(), String::new());

    if let Some(name) = cli.input.as_deref() {
        payload.0 = name.to_string();
    } else {
        return Err(TektonError::Reason("Missing input file name".to_string()));
    }

    if let Some(n) = cli.output.as_deref() {
        payload.1 = n.to_string()
    } else {
        return Err(TektonError::Reason("Missing output file name".to_string()));
    }
    Ok(payload)
}
