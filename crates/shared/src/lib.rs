use std::{ops::Range, path::PathBuf};

use clap::Parser;

pub type Span = Range<usize>;

#[derive(Parser)]
pub struct Args {
    /// Path pointing to the input file
    #[arg(help = "Path na nagtuturo sa input file")]
    pub input: PathBuf,

    /// Path pointing to where the output file should be put (if given a directory) or to output a
    /// file specified (if given a file). Produces `./<input_filename> (for linux) or
    /// .\<input_filename>.exe (for windows)` by default.
    #[arg(
        short,
        long,
        help = "Path na nagtuturo kung saan ilalagay ang output na file (kung directory ang binigay) o magououtput ng inilagay na file (kung file ang binigay). Gagawa ng `./<input_filename>) (sa linux) o .\\<input_filename>.exe (sa windows) sa karaniwan"
    )]
    pub output: Option<PathBuf>,
}
