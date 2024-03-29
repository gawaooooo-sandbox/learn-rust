use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(default_value_t = 2023)]
    port: u16,
}

fn main() {
    let cli = Cli::parse();

    println!("port: {:?}", cli.port);
}
