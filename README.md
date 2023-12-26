
# Shellify

A program written in rust to help facilitate the process of writing shellcodes

## Roadmap

- Add badchars highlighting 

- Add local shellcode debugging


## Installation

Clone repository and build as a cargo project

```bash
  git clone https://github.com/2sKq/Shellify
  cd Shellify
  cargo build --release
```
The compiled project will be located in ~/target/release/shellify
    
## Usage

Usage: shellify --path <PATH> --format <FORMAT>

Options:

      -p, --path <PATH>
      -f, --format <FORMAT>  [default: payload] [possible values: c, python, rust, payload, hex, hex-c]
      -h, --help             Print help
      -V, --version          Print version