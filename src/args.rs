extern crate clap;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct TektonArgs {
    #[clap(subcommand)]
    pub entity_type: TektonEntity,
}

#[derive(Debug, Subcommand)]
pub enum TektonEntity {
    /// Convert Snippets
    Convert(ConversionCommand),
}

#[derive(Debug, Args)]
pub struct ConversionCommand {
    /// The input filename
    pub input_filename: String,
    /// The output filename
    pub output_filename: String,
}
