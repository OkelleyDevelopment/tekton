use crate::{errors::TektonError, models::args::ConversionCommand};

use super::{
    composer::composer,
    utils::{get_filetype_extension, write_to_file},
};

/// The conversion handler ment to control the conversion portion of the program.
pub fn convert_handler(convert: ConversionCommand) -> Result<(), TektonError> {
    let file_extensions = (
        get_filetype_extension(&convert.input_filename).unwrap(),
        get_filetype_extension(&convert.output_filename).unwrap(),
    );
    let output = convert.output_filename.to_string();
    let snippets = composer(
        &convert.input_filename,
        file_extensions,
        convert.interactive.is_some(),
    )?;
    write_to_file(output, snippets);
    Ok(())
}

#[test]
fn test_opt() {
    let s = Some(3);
    assert_eq!(s.is_none(), false);

    assert_eq!(s.is_some(), true);
}
