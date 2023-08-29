
use std::{path::PathBuf, io, fs::File, io::BufRead};

use fend_core as fend;
use clap::Parser;

const EQUALS: &str = "#=";

/// Proof of concept to use fend to eval text file with calculations
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// input file
    #[clap(index = 1)]
    file: PathBuf,
}

fn main() {

    let args: Args = Args::parse();
    let file = File::open(args.file).unwrap();
    let mut context = fend::Context::new();

    for line in io::BufReader::new(file).lines().into_iter() {
        let line = line.unwrap();
        let parts: Vec<_> = line.split(EQUALS).collect();
        let expr = parts[0];
        if let Ok(result) = fend::evaluate(expr, &mut context) {
            if parts.len() > 1 {
                println!("{} {} {}", expr, EQUALS, result.get_main_result());
            } else {
                println!("{}", line)
            }
        } else {
            println!("{}", line);
        }
    }
}
