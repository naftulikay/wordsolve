extern crate structopt;

mod cli;
mod dictionary;

use structopt::StructOpt;

fn main() {
    let args = cli::Args::from_args();

    for allowed_word in
        dictionary::get_word_list(args.min_size, args.max_size, args.available_chars)
    {
        println!("{}", allowed_word);
    }
}
