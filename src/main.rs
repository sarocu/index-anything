use std::io::{self, BufRead, Write};
use std::fs;
use byteorder::{BigEndian, WriteBytesExt};

#[derive(Debug)]
pub struct IndexArgs {
    file: std::path::PathBuf,
    idx: std::path::PathBuf,
    encoding: Option<String>,
    take: Option<u32>,
    start: Option<u32>,
}

// help text:
const HELP: &str = "\
    App

    USAGE:
    idx --file PATH

    FLAGS:
    -h, --help            Prints help information

    OPTIONS:
    --file      PATH           Sets the text file to index
    --idx       PATH           Sets the output path of the created index, or the index to use for fetching
    --encoding  STRING         Sets an optional encoding (defaults to utf8)
    --take      NUMBER         Returns a set of lines starting at the line provided by `--start` (optional)
    --start     NUMBER         The beginning line to fetch from, used with `--take`, zero indexed (optional)

    ARGS:
    <INPUT>
    ";

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    println!("{:#?}", args);

    // create a new index file, it's cool to panic here:
    let f = fs::File::create(args.idx).expect("could not create index file");
    let mut writer = io::BufWriter::new(f);

    // open input file, also cool to panic:
    let input = fs::File::open(args.file).expect("could not open input file!");
    let mut reader = io::BufReader::new(input);

    let mut eof = false;
    let mut pos: u64 = 0; // byte position
    let mut buf = String::new(); // string buffer

    while !eof {
        let len = reader.read_line(&mut buf);
        match len {
            Err(_) => eof = true,
            Ok(l) => {
                if l == 0 {
                    eof = true
                } else {
                    pos = pos + l as  u64;
                    writer.write_u64::<BigEndian>(pos).expect("could not write byte offset position")
                }
            }
        }
        // don't slurp memory:
        buf.clear();
    }
    writer.flush().expect("couldn't successfully flush index");
}

fn parse_args() -> Result<IndexArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = IndexArgs {
        file: pargs.value_from_str("--file")?,
        idx: pargs.value_from_str("--idx")?,
        encoding: pargs.opt_value_from_str("--encoding")?,
        take: pargs.opt_value_from_str("--take")?,
        start: pargs.opt_value_from_str("--start")?
    };

    Ok(args)
}
