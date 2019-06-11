extern crate structopt;

mod cli;
mod dictionary;

use dictionary::Dictionary;

use structopt::StructOpt;

fn main() {
    let args = cli::Args::from_args();

    // construct a dictionary of min_size..max_size (inclusive) word lengths
    let dictionary = Dictionary::new(args.min_size, args.max_size);

    // filter the dictionary to only contain solutions given the following available characters
    let mut solutions: Vec<String> = dictionary
        .solutions(&args.available_chars)
        .into_iter()
        .collect();

    // sort our solutions
    solutions.sort();

    // print out our solutions
    solutions.iter().for_each(|s| println!("{}", s));
}
