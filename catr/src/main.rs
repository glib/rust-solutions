use clap::Parser;
use std::fs::File;
use std::io::Result;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of cat
struct Args {
    ///input files
    #[arg(default_value = "-")]
    files: Vec<String>,

    ///Number lines
    #[arg(short, long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,

    /// Number non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => {
                let mut real_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if args.number_lines {
                        let line_num = line_num + 1;
                        println!("{line_num:>6}\t{line}");
                    } else if args.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                            continue;
                        }
                        real_num += 1;
                        println!("{real_num:>6}\t{line}");
                    } else {
                        println!("{line}");
                    }
                    // match line {
                    //     Err(err) => eprintln!("Failed to read line: {err}"),
                    //     Ok(line_string) => println!("{line_string}"),
                    // }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprint!("{e}");
        std::process::exit(1);
    }
}
