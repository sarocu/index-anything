# Index Anything
Well, text files at least.

This will take a file, loop through and identify the position of each newline character, writing that position to the index file.

Influenced greatly by https://github.com/BurntSushi/rust-csv

## Dev stuff
```bash
# build the project
cargo build

# lint n fix
cargo fmt
cargo fix

# executable at target/debug/idx
# move it to the path:
cp target/debug/idx /usr/local/bin/
```

## Usage:
```bash
idx --help

# index a file
idx --file ~/Documents/sample-files/100000.csv --idx ./index.idx

# fetch rows from that file
idx --file ~/Documents/sample-files/100000.csv --idx ./index.idx --start 100 --take 50

# save a slice of rows to a file
idx --file ~/Documents/sample-files/100000.csv --idx ./index.idx --take 10 --start 100 > 10.txt
```
