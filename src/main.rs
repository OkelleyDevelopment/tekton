use clap::Parser;
use tekton::args::{TektonArgs, TektonEntity};
use tekton::composer::composer::composer;
use tekton::errors::TektonError;
use tekton::utils::get_extension_from_filename;
use tekton::utils::write_to_file;
/// Entry point to the CLI App
fn main() -> Result<(), TektonError> {
    let args = TektonArgs::parse();
    let output;

    let snippets = match args.entity_type {
        TektonEntity::Convert(convert) => {
            let file_extensions = (
                get_extension_from_filename(&convert.input_filename).unwrap(),
                get_extension_from_filename(&convert.output_filename).unwrap(),
            );
            output = convert.output_filename.to_string();
            composer(&convert.input_filename, file_extensions)
        }
    };

    match snippets {
        Ok(snippets) => Ok(write_to_file(output, snippets)),
        Err(e) => Err(e),
    }
}
