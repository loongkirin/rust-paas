use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "paas-cli")]
#[command(version = "0.1.0")]
#[command(about = "CLI for rust-paas project")]
struct Args {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        println!("Verbose mode enabled");
    } else {
        println!("paas-cli {}", env!("CARGO_PKG_VERSION"));
    }
}
