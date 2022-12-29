use crate::{
    core::composer::{composer, multiprefix_composer},
    errors::TektonError,
    models::args::SortCommand,
    utils::{crawl_files, get_filetype_extension, write_to_file},
};

// A named constant for the sort option
const SORT: &str = "tekton-sort";

/// Hanlder for the Sorting Mechanism
pub fn sort_handler(sort: SortCommand) -> Result<(), TektonError> {
    let files = crawl_files(sort.path, sort.crawl);
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

    // If there are files to fix AND the sort is interactive, then  we will execute this chunk of code.
    if sort.interactive.is_some() && !files_to_try.is_empty() {
        for name in files_to_try.iter() {
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
    }
    println!("[Tekton]: Files sorted: {}", file_count);

    Ok(()) // We can exit with an ok since *not writing* doesn't need to end the program.
           // inform and move on.
}
