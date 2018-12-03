# Colours
Display a preview of terminal colours in a pleasing box. It's useful to have a
way to test terminal output without writing a one-liner in whatever shell you
happen to be using.

## Options
  * `--bright` show both normal and bright colours
  * `--help` displays the help message

## Develop
  1. Clone the repository: `git clone git@github.com:colinstein/colours`.
  2. Make some changes to the code `vi colours/src/colours.rs`.
  3. Build (debug mode) `cargo build`.
  4. Run tests `cargo test`.
  5. Run Program `cargo run`.
  6. Open a pull request.

## Compile
`cargo build --release`.

## Install
`cargo build --release && cp target/release/colours /usr/local/bin.`
