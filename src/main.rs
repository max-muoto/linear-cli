use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
}
