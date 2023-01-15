//! The entry point into the conversion of snippets

use crate::{
    core::composer::composer,
    errors::TektonError,
    models::args::ConversionCommand,
    utils::{get_filetype, write_to_file},
};

const INTERACTIVE: bool = true;

/// The conversion handler ment to control the conversion portion of the program.
pub fn convert_handler(convert: ConversionCommand) -> Result<(), TektonError> {
    let file_extensions = (
        get_filetype(&convert.input_filename).unwrap(),
        get_filetype(&convert.output_filename).unwrap(),
    );
    let output = convert.output_filename.to_string();
    // Conversion is always interactive
    let snippets = composer(&convert.input_filename, file_extensions, INTERACTIVE)?;
    write_to_file(output.clone(), snippets);
    println!("[Tekton]: Wrote snippets to {}", output);
    Ok(())
}
