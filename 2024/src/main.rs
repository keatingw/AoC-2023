use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    day: u8,
}

fn main() {
    let cli = Cli::parse();
    println!("day={}", cli.day);
}
