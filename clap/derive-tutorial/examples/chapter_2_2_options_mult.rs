use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(short, long)]
    name: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name);
}
