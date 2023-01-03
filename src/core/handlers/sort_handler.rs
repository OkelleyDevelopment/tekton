use crate::{
    core::composer::{composer, multiprefix_composer},
    errors::TektonError,
    models::args::SortCommand,
    utils::{crawl_files, get_filetype, write_to_file},
};

// A named constant for the sort option
const SORT: &str = "tekton-sort";

/// Hanlder for the Sorting Mechanism
pub fn sort_handler(sort: SortCommand) -> Result<(), TektonError> {
    // the counter variable
    let mut file_count = 0;
    // Determines if the user will need to be invovled or not.
    let is_interactive = sort.interactive.is_some();
    let mut is_directory = false;
    // The candidates of files to potentially sort
    let files = crawl_files(sort.path, sort.crawl);

    let files_to_correct: Vec<Option<String>> = files
        .iter()
        .filter(|file| {
            if file.metadata().unwrap().is_dir() {
                is_directory = true;
            }
            !file.metadata().unwrap().is_dir()
        })
        .map(|file| {
            file_count += 1;
            let fname: String = file.clone().into_os_string().to_str().unwrap().to_string(); // this isn't the best thing on Earth. Now Pluto? Perhaps.
            let extensions = (get_filetype(&fname).unwrap(), SORT);
            match composer(&fname, extensions, is_interactive) {
                Ok(snippets) => {
                    write_to_file(fname, snippets);
                    None
                }
                Err(_e) => {
                    file_count -= 1;
                    Some(fname)
                }
            }
        })
        .collect();

    if is_interactive {
        for name in files_to_correct.iter().flatten() {
            file_count += 1;
            let snippets = multiprefix_composer(name);
            match snippets {
                Ok(s) => write_to_file(name.to_string(), s),
                Err(e) => {
                    println!("[Tekton Error]: Unable to process file: `{}`", &name);
                    println!("{}\n", e);
                    file_count -= 1;
                }
            }
        }
    } else {
        let mut error_message: String = "".to_string();
        // Displays a warning message if there are issues
        if !files_to_correct.is_empty() {
            error_message += "[ Warn ]: Issues found with these files: ";
            for name in files_to_correct.into_iter().flatten() {
                error_message = error_message + "\n\t" + &name;
            }

            println!(
                "{}\n\n[Tekton]: Run with interactive mode to be able to fix any issues in the files listed.",
                error_message
            );
        } else if is_directory && !is_interactive {
            println!(
                "[ Warn ]: You provided a directory, without the `crawl` argument.\n\t  Try again."
            );
        } else {
            println!("[Tekton]: No errors detected in the file(s).");
        }
    }
    println!("[Tekton]: Files sorted: {}", file_count);

    Ok(())
}
