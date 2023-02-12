use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(name = "My RPN program")]
#[command(version = "1.0.0")]
#[command(author = "Your name")]
#[command(about = "Super awesome sample RPN calculator")]
struct Opts {
    /// Sets the level of verbosity
    #[arg(short, long)]
    verbose: bool,

    /// Fomulas written in RPN
    #[arg(value_name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    // match opts.formula_file {
    //     Some(file) => println!("File specified: {}", file),
    //     None => println!("No file specified."),
    // }
    // println!("Is verbosity specified?: {}", opts.verbose);

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
    } else {
        // ファイルを指定しなかった場合
        println!("No file is specified");
    }
}
