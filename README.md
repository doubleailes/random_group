# random_group

A CLI tool that randomly splits a list of people (one name per line in a text file) into balanced groups.

## Usage

```bash
random_group <FILE> --num-groups <N>
random_group <FILE> --group-size <S>
```

Exactly one of `--num-groups` or `--group-size` is required.

### Arguments

| Argument           |Short | Description                                |
|--------------------|------|--------------------------------------------|
| `<FILE>`           |      | Path to a text file with one name per line |
| `--num-groups <N>` | `-n` | Split into N groups of roughly equal size  |
| `--group-size <S>` | `-s` | Split into groups of at most S people      |

## Examples

Split `sample.txt` into 4 balanced groups:

```bash
$ random_group sample.txt --num-groups 4
Group 1:
  - Pauline
  - Phillippe
  - Laurent
  - Julien
  - Justin
  - Pierre-Marie
  - Mohamed
  - Lou
Group 2:
  ...
```

Split `sample.txt` into groups of 5:

```bash
$ random_group sample.txt --group-size 5
Group 1:
  - Roger
  - Vanessa
  - Jasmin
  - Olivier
  - Mat
Group 2:
  ...
```

## Input file format

A plain text file with one name per line. Blank lines are ignored.

```text
Alice
Bob
Charlie
Diana
```

## Build

```text
cargo build --release
```

The binary will be at `target/release/random_group`.

## Dependencies

- [clap](https://github.com/clap-rs/clap) — CLI argument parsing
- [rand](https://github.com/rust-random/rand) — Random shuffling
