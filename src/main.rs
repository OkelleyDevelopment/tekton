extern crate walkdir;
use clap::Parser;
use tekton::args::{TektonArgs, TektonEntity};
use tekton::core::convert_handler::convert_handler;
use tekton::core::sort_handler::sort_handler;
use tekton::errors::TektonError;

/// Entry point to the CLI App
fn main() -> Result<(), TektonError> {
    let args = TektonArgs::parse();

    match args.entity_type {
        TektonEntity::Convert(convert) => convert_handler(convert),
        TektonEntity::Sort(sort) => sort_handler(sort),
    }
}
