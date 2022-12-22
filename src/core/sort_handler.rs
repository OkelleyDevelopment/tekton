use std::path::PathBuf;

use walkdir::WalkDir;

use crate::{
    args::SortCommand,
    core::composer::composer,
    errors::TektonError,
    utils::{get_filetype_extension, write_to_file},
};

// A named constant for the sort option
const SORT: &str = "tekton-sort";

/// Hanlder for the Sorting Mechanism
pub fn sort_handler(sort: SortCommand) -> Result<(), TektonError> {
    let files = parse_files(sort);
    let mut file_count = 0;
    for file in files {
        if file.metadata().unwrap().is_dir() {
            println!(
                "Skipping directory. Supply a crawl argument if you wish to descend into {}.",
                file.as_path().display()
            );
            continue;
        }
        file_count += 1;
        let fname: String = file.into_os_string().to_str().unwrap().to_string(); // this isn't the best thing on Earth.
        let extensions = (get_filetype_extension(&fname).unwrap(), SORT);
        match composer(&fname, extensions) {
            Ok(snippets) => {
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

/// Private helper function to process what files to crawl.
fn parse_files(sort: SortCommand) -> Vec<PathBuf> {
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
    files
}
