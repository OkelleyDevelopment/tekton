use std::path::PathBuf;

use walkdir::WalkDir;

use crate::{
    core::composer::{composer, last_composer},
    errors::TektonError,
    models::args::SortCommand,
    utils::{get_filetype_extension, write_to_file},
};

// A named constant for the sort option
const SORT: &str = "tekton-sort";

/// Hanlder for the Sorting Mechanism
pub fn sort_handler(sort: SortCommand) -> Result<(), TektonError> {
    let files = parse_files(sort);
    let mut files_to_try: Vec<String> = Vec::new();
    println!("[Tekton]: Files counted: {}", files.len());
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
        // println!("[Tekton]: {}", fname);
        match composer(&fname, extensions) {
            Ok(snippets) => {
                write_to_file(fname, snippets);
            }
            Err(_) => {
                file_count -= 1;
                files_to_try.push(fname);
            }
        }
    }

    // We only write to the file if there are no misconfigured snipptets
    if !files_to_try.is_empty() {
        for name in files_to_try.iter() {
            file_count += 1;
            let snippets = last_composer(name);
            match snippets {
                Ok(s) => write_to_file(name.to_string(), s),
                Err(e) => {
                    println!("[Tekton Error]: Unable to process file: `{}`", &name);
                    println!("{}\n", e);
                    file_count -= 1;
                }
            }
        }
    }
    println!("[Tekton]: Files sorted: {}", file_count);

    Ok(()) // We can exit with an ok since *not writing* doesn't need to end the program.
           // inform and move on.
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
