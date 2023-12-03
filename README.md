# Advent of Code 2023

My solution for advent of code 2023 in Rust.

## To run and time everything

```bash
cargo run
```

## To test or time a specific day

```
cargo run -- <day_number> [--time]
```

## Template

Initialize with little bash script to create directories and files for each day `./init_files.sh`.

```
src/
    day01/
        mod.rs # code goes here
        input.txt
        example1.txt
        example2.txt
    day02/
        ...
```

Each day use a generic trait `DaySolution` providing functions for solving both parts and constant for expected answers to examples of the two parts.

These 2 functions will be used for testing and timing.
We use macros to avoid declaring 25 modules and match with runtime known integer for selecting day.

Also there is a little script to download current input using curl. It supposed to extract cookie from your browser in a `cookie.txt` file. It's expected to be used like this `./get_input.sh <day_number>`
