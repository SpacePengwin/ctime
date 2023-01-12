# ctime
`ctime` or Crab Time. for rustaceans is a command line utility to get run-time from a Windows, MacOS, or Linux system program. All of these different systems have a different default `time` utility. The goal of this was to use one program for all three when benchmarking a program's run-time.

# Installation
The package is currently avaliable through `cargo` Rust's package manager.

You can install it with: `cargo install ctime`

The crates.io page is [here](https://crates.io/crates/ctime)

# Usage
Prepend this program to another command-line program to time the total run-time of the timed program. Use `--silence-output` or `-s` to not display stdout of the program being timed.

Basic example:
`ctime echo 'HELLO'`

This would output:
```
$ ctime echo 'HELLO'
HELLO

Time elapsed: 427.253µs
```

Using with an actual benchmark program, like the [Python `mandlebrot` set](https://benchmarksgame-team.pages.debian.net/benchmarksgame/program/mandelbrot-python3-7.html) would look like this:
```
$ ctime python3 mandlebrot.py 16000 -s
Time elapsed: 23.450438757s
```

Running on a Windows Machine via Powershell:
```
PS $> ctime python3 .\mandlebrot.py 16000 -s
Time elapsed: 24.412431292s
```

Comparing to the "real" value from `time`:
```
$ time python3 mandlebrot.py 16000 > /dev/null
real    0m23.590s
user    9m21.693s
sys     0m0.924s
```
