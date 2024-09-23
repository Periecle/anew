# anew
[![CI](https://github.com/Periecle/anew/actions/workflows/ci.yml/badge.svg)](https://github.com/anew/unique_lines/actions/workflows/ci.yml)

Basically it's a Rust rewrite of [Tomnomnom anew](https://github.com/tomnomnom/anew) originally written in Go.

70% of the Readme is a tomnomnom work.
Append lines from stdin to a file, but only if they don't already appear in the file.
Outputs new lines to `stdout` too, making it a bit like a `tee -a` that removes duplicates.

## Usage Example

Here, a file called `things.txt` contains a list of numbers. `newthings.txt` contains a second
list of numbers, some of which appear in `things.txt` and some of which do not. `anew` is used
to append the latter to `things.txt`.


```
▶ cat things.txt
Zero
One
Two

▶ cat newthings.txt
One
Two
Three
Four

▶ cat newthings.txt | anew things.txt
Three
Four

▶ cat things.txt
Zero
One
Two
Three
Four

```

Note that the new lines added to `things.txt` are also sent to `stdout`, this allows for them to
be redirected to another file:

```
▶ cat newthings.txt | anew things.txt > added-lines.txt
▶ cat added-lines.txt
Three
Four
```

## Flags

- To view the output in stdout, but not append to the file, use the dry-run option `-d` or `--dry-run`.
- To append to the file, but not print anything to stdout, use quiet mode `-q` or `--quiet`.
- To trim the lines before saving to file, use trim `-t` or `--trim`.

## Installation

Download the binary for your platform from the [Releases](https://github.com/Periecle/anew/releases) page.

### From Source

```bash
git clone https://github.com/Periecle/anew.git
cd anew
cargo build --release