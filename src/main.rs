use clap::{Parser, ValueEnum};
use colored::*;
use object::{File, Object, ObjectSection, SectionKind};

use std::error::Error;
use std::fs;
use std::io::Read;

#[derive(Parser)]
#[command(name = "Shellify")]
#[command(author = "1sKq")]
#[command(version = "1.0")]
#[command(about = "Simple program meant to facilitate the process of hand-writing shellcodes")]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long, value_enum, default_value = "payload")]
    format: Format,
}

fn parse_dot_text(file: File) -> Result<&[u8], Box<dyn Error>> {
    for section in file.sections() {
        if section.kind() == SectionKind::Text {
            let data = section.data()?;
            return Ok(data);
        }
    }
    return Err("No .text section found in the provided binary".into());
}

fn format_ops(opcodes: &Vec<String>, format: Format, file_path: &str) {
    println!(
        "{} {}",
        "\nGenerated payload from:".bold(),
        file_path.bright_black()
    );
    println!(
        "{} {} {}",
        "Payload Length:".bold(),
        opcodes.len().to_string().bright_black(),
        "bytes".bright_black()
    );

    match format {
        Format::C => {
            println!("{} {}\n", "Format:".bold(), "c".bright_black());

            print!("unsigned char buf[] = \"");
            for op in opcodes.iter() {
                print!("\\x{}", op);
            }
            print!("\";")
        }

        Format::Python => {
            println!("{} {}\n", "Format:".bold(), "python".bright_black());

            print!("payload = b\"");
            for op in opcodes.iter() {
                print!("\\x{}", op);
            }
            print!("\"")
        }

        Format::Rust => {
            println!("{} {}\n", "Format:".bold(), "rust".bright_black());

            print!("let payload = b\"");
            for op in opcodes.iter() {
                print!("\\x{}", op);
            }
            print!("\";")
        }

        Format::Payload => {
            println!("{} {}\n", "Format:".bold(), "payload".bright_black());

            for op in opcodes.iter() {
                print!("\\x{}", op);
            }
        }

        Format::Hex => {
            println!("{} {}\n", "Format:".bold(), "hex".bright_black());

            for op in opcodes.iter() {
                print!("0x{} ", op);
            }
        }

        Format::HexC => {
            println!("{} {}\n", "Format:".bold(), "hex comma".bright_black());

            for op in opcodes.iter() {
                print!("0x{}, ", op);
            }
        }
    }
    print!("\n");
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();
    let file_path: String = args.path;
    let format: Format = args.format;

    let mut ops: Vec<String> = Vec::new();

    let mut file = match fs::File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!(
                "\n{} {}",
                "Error occured while opening file:".red(),
                err.to_string().white().italic()
            );
            std::process::exit(1)
        }
    };

    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;

    // Parse the binary as an object file
    let file = match File::parse(&buf[..]) {
        Ok(file) => file,
        Err(err) => {
            eprintln!(
                "\n{} {}",
                "Error occured while parsing the binary:".red(),
                err.to_string().white().italic()
            );
            std::process::exit(1)
        }
    };

    let raw: &[u8] = parse_dot_text(file)?;

    // Extract formatted opcodes from the raw data
    for chunk in raw.iter() {
        let op = format!("{:02x}", chunk);
        ops.push(op);
    }

    //print formatted opcodes!
    format_ops(&ops, format, &file_path);

    Ok(())
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    C,
    Python,
    Rust,
    Payload,
    Hex,
    HexC,
}
