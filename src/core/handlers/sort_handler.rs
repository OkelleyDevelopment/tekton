//! The entry point for sorting of snippets 
//! which at this time assumes the sorting of JSON.

use std::path::PathBuf;

use crate::{
    core::composer::{composer, multiprefix_composer},
    errors::TektonError,
    models::args::SortCommand,
    utils::{crawl_files, get_filetype, write_to_file},
};

// A named constant for the sort option
const SORT_COMMAND_SLICE: &str = "tekton-sort";

/// Hanlder for the Sorting Mechanism
///
/// Arguments
/// - `sort` : the parameters from the CLI
///
/// Returns
/// - An ok result or a TektonError
///
pub fn sort_handler(sort: SortCommand) -> Result<(), TektonError> {
    let mut manager: SortConfigManager = SortConfigManager::new(sort.crawl.is_some());
    manager.set_interactive(sort.interactive);

    let crawled_files_and_dirs = crawl_files(sort.path, sort.crawl);
    let filtered_files: Vec<&std::path::PathBuf> = crawled_files_and_dirs
        .iter()
        .filter(|buf| {
            if buf.metadata().unwrap().is_dir() {
                manager.is_path_directory = true;
            }
            !buf.metadata().unwrap().is_dir()
        })
        .collect();

    if manager.is_path_directory && !manager.is_crawling {
        println!("[ WARN ]: Provided a directory without crawling.\n\t  Try again.");
        return Ok(());
    }

    let files_to_correct: Vec<String> = manager.first_pass_sorting(filtered_files);

    if !manager.is_interactive {
        if let Some(message) = manager.gen_files_to_correct_string(files_to_correct) {
            println!("{}", message);
        }
    } else {
        for name in files_to_correct.iter() {
            let snippets = multiprefix_composer(name);
            match snippets {
                Ok(s) => {
                    manager.file_count += 1;
                    write_to_file(name.to_string(), s)
                }
                Err(e) => {
                    println!(
                        "[Tekton Error]: Unable to process file: `{}`\n\t{}",
                        &name, e
                    );
                    manager.corrections_passed = false;
                }
            }
        }
        if manager.corrections_passed {
            println!("[Tekton]: No errors detected in the file(s).");
        }
    }

    println!("[Tekton]: Files sorted: {}", manager.file_count);

    Ok(())
}

// The Configuration manager for the sorting
// utility.
struct SortConfigManager {
    // Files sorted
    pub file_count: usize,
    // Boolean for user input
    pub is_interactive: bool,
    // Boolean for recursive descent into directories
    pub is_crawling: bool,
    // A boolean for path being a directory
    pub is_path_directory: bool,
    // Flag to determine if the corrections were successful
    pub corrections_passed: bool,
}

impl SortConfigManager {
    /// Function to create a new manager
    pub fn new(is_crawling: bool) -> Self {
        Self {
            file_count: 0,
            is_interactive: false,
            is_crawling,
            is_path_directory: false,
            corrections_passed: true,
        }
    }
    /// Method to set bool flag
    pub fn set_interactive(&mut self, input: Option<String>) {
        self.is_interactive = match input {
            Some(val) => val == "yes",
            None => false,
        };
    }

    pub fn first_pass_sorting(&mut self, filtered_files: Vec<&PathBuf>) -> Vec<String> {
        let return_list: Vec<String> = filtered_files
            .iter()
            .filter_map(|file| {
                let fname: String = <&std::path::PathBuf>::clone(file)
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_string();
                let extensions = (get_filetype(&fname).unwrap(), SORT_COMMAND_SLICE);
                match composer(&fname, extensions, self.is_interactive) {
                    Ok(snippets) => {
                        write_to_file(fname, snippets);
                        self.file_count += 1;
                        None
                    }
                    Err(_e) => Some(fname),
                }
            })
            .collect();

        return_list
    }

    pub fn gen_files_to_correct_string(&self, files_to_correct: Vec<String>) -> Option<String> {
        let mut error_message: String = "".to_string();

        if files_to_correct.is_empty() {
            error_message += "[Tekton]: No errors detected in the file(s).";
        } else {
            error_message += "[ Warn ]: Issues found with these files: ";
            for name in files_to_correct.iter() {
                error_message = error_message + "\n\t" + name;
            }
            error_message += "\n\n[Tekton]: Run with interactive mode to be able to fix any issues in the files listed.";
        }

        if error_message.is_empty() {
            None
        } else {
            Some(error_message)
        }
    }
}
