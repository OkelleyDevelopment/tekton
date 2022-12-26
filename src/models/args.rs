//! The commands for the tekton cli program
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
    /// Sorting JSON
    Sort(SortCommand),
}

#[derive(Debug, Args)]
pub struct ConversionCommand {
    /// The input filename
    pub input_filename: String,
    /// The output filename
    pub output_filename: String,
}

#[derive(Debug, Args)]
pub struct SortCommand {
    /// Path to the snippets
    pub path: String,
    /// If present, then the path should be to a directory
    pub crawl: Option<String>,
}
