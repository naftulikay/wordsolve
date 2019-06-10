use structopt::StructOpt;

/// Solve crossword puzzles given a list of possible characters.
#[derive(Debug, StructOpt)]
pub struct Args {
    /// The maximum word size to search for answers. Set this to the smallest word size possible to
    /// ensure best performance and memory usage.
    #[structopt(short = "M", long = "max-size", default_value = "6")]
    pub max_size: u8,
    /// The minimum word size to search for answers. Words shorter than this number will not be
    /// included in results.
    #[structopt(short = "m", long = "min-size", default_value = "3")]
    pub min_size: u8,
    /// The list of letters available for the given puzzle. Letters may occur more than once.
    pub available_chars: String,
}
