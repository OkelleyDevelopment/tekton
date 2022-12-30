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
    let mode = sort.interactive.is_some();
    // The candidates of files to potentially sort
    let files = crawl_files(sort.path, sort.crawl);

    println!("[Tekton]: Files counted: {}", files.len());

    let files_to_try: Vec<Option<String>> = files
        .iter()
        .filter(|file| !file.metadata().unwrap().is_dir())
        .map(|file| {
            file_count += 1;
            let fname: String = file.clone().into_os_string().to_str().unwrap().to_string(); // this isn't the best thing on Earth.
            let extensions = (get_filetype(&fname).unwrap(), SORT);
            match composer(&fname, extensions, mode) {
                Ok(snippets) => {
                    write_to_file(fname, snippets);
                    None
                }
                Err(_) => {
                    file_count -= 1;
                    Some(fname)
                }
            }
        })
        .collect();

    if sort.interactive.is_none() {
        for name in files_to_try.iter().flatten() {
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

    Ok(())
}
