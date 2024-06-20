# File Parser

The file parser is a simple program that reads the contents of a given file, searches for the word and replaces it with a given word.

## How to use

### Clone the Repository

```bash
git clone https://github.com/KarnesTH/file_parser.git
cd file_parser
```

Before you can use the program, you should to be compile the program.

### With cargo

```bash
cargo build
```

> You need to change the directory after the build before you can use the program.
>
> `cd /target/debug`
>
> You can't use `cargo run` because you need to give the arguments.

### With rustc

```bash
cd src
rustc main.rs
```

## Usage

### Syntax

```bash
./file_parser <pattern> <replace> <input_filename> <output_filename>
```

### Example

```bash
./file_parser edition std /path/to/the/input.txt /path/for/the/output.txt
```
