use clap::Parser;
use std::env;

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
    
    if env::var("RUST_LOG").is_err() {
        if args.verbose {
            env::set_var("RUST_LOG", "debug");
        } else {
            env::set_var("RUST_LOG", "info");
        }
    }
    
    env_logger::init();
    
    println!("paas-cli {}", env!("CARGO_PKG_VERSION"));
}
