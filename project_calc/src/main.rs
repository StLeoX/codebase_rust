use calc_syntax::parse::CalcParser;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    /// The path of file to calculate
    #[clap(short, long)]
    pub file: PathBuf,

    /// The expression to calculate
    #[clap(short, long)]
    pub expr: String,
}

fn main() {
    let args = Args::parse();
    let file = args.file;
    let input: String = std::fs::read_to_string(file).expect("Unable to read input file");
    let parser = CalcParser::new(input);
    // ??
    let result: f64 = parser.parse().visit();
    println!("calc result: {}", result);
}
