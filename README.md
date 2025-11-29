# Advent of Code 2025
My solutions for advent of code 2025 in Rust.

https://adventofcode.com/2025 

## Running

### Input from website
Take the session cookie from browser (using network developer tool and look at request header `cookie: session=<session_value>`).
Set the `AOC_SESSION` env var to the session value then run:
```
cargo run -p day<value>
```
where `<value>` is aoc day.

### Input from file
Input can be taken from a file by setting the `-i <input_file>` argument:
```
cargo run -p day<value> -- -i <input_file>
```

### Running single part
Running only part one or part two can be done by setting the `-o` or `-t` arguments;
```
cargo run -p day<value> -- -o
```

