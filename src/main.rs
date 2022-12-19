extern crate walkdir;
use clap::Parser;
use std::path::PathBuf;
use tekton::args::{TektonArgs, TektonEntity};
use tekton::core::composer::composer;
use tekton::errors::TektonError;
use tekton::utils::{get_extension_from_filename, write_to_file};
use walkdir::WalkDir;

// A named constant for the sort option
const SORT: &str = "tekton-sort";

/// Entry point to the CLI App
fn main() -> Result<(), TektonError> {
    let args = TektonArgs::parse();
    let output;

    match args.entity_type {
        TektonEntity::Convert(convert) => {
            let file_extensions = (
                get_extension_from_filename(&convert.input_filename).unwrap(),
                get_extension_from_filename(&convert.output_filename).unwrap(),
            );
            output = convert.output_filename.to_string();
            let snippets = composer(&convert.input_filename, file_extensions)?;
            write_to_file(output, snippets);
            Ok(())
        }
        TektonEntity::Sort(sort) => {
            let mut files: Vec<PathBuf> = Vec::new();
            if sort.crawl.is_some() {
                for file in WalkDir::new(sort.path)
                    .into_iter()
                    .filter_map(|file| file.ok())
                {
                    if file.metadata().unwrap().is_file() {
                        files.push(file.path().to_path_buf());
                    }
                }
            } else {
                files.push(PathBuf::from(sort.path));
            }

            let mut file_count = 0;
            for file in files {
                if file.metadata().unwrap().is_dir() {
                    println!("Skipping directory. Supply a crawl argument if you wish to descend into {}.",file.as_path().display());
                    continue;
                }
                file_count += 1;
                let fname: String = file.into_os_string().to_str().unwrap().to_string(); // this isn't the best thing on Earth.
                //println!("Currently sorting: {}", fname);
                let extensions = (get_extension_from_filename(&fname).unwrap(), SORT);
                match composer(&fname, extensions) {
                    Ok(snippets) => {
                        //println!("Writing the file post sort");
                        write_to_file(fname, snippets);
                    }
                    Err(e) => {
                        println!("Error: Unable to sort file: `{}`, \t\n{:?}.", &fname, e);
                    }
                }
            }
            println!("Files sorted: {}", file_count);
            Ok(())
        }
    }
}
