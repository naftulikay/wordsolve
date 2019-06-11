# wordsolve [![Build Status][travis.svg]][travis]

A solver for them crossword games.

Special shout-out and thanks to [dwyl/english-words][english-dictionary] for the English dictionary used by this
application.

## Usage

This crate provides a single binary, `wordsolve`:

```
USAGE:
    wordsolve [OPTIONS] <available_chars>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -M, --max-size <max_size>    The maximum word size to search for answers. Set this to the smallest word size
                                 possible to ensure best performance and memory usage. [default: 6]
    -m, --min-size <min_size>    The minimum word size to search for answers. Words shorter than this number will not be
                                 included in results. [default: 3]

ARGS:
    <available_chars>    The list of letters available for the given puzzle. Letters may occur more than once.
```

To solve a standard 3-6 letter puzzle with `tootal` as the allowed characters:

```
$ wordsolve -m 3 -M 6 tootal
alo
alt
alto
att
lao
lat
...
```

Note that this application is only as good as its dictionary, and many puzzles accept a quite limited set of words.
Your mileage may vary.

## License

Licensed at your discretion under either:

 - [Apache Software License, Version 2.0](./LICENSE-APACHE)
 - [MIT License](./LICENSE-MIT)

 [english-dictionary]: https://github.com/dwyl/english-words
 [travis]: https://travis-ci.org/naftulikay/wordsolve
 [travis.svg]: https://travis-ci.org/naftulikay/wordsolve.svg?branch=master
