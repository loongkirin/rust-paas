use clap::{Parser, Subcommand};
use rpassword::read_password;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

const TOKEN_FILE: &str = ".config/rust-paas/token";

fn get_token_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(TOKEN_FILE)
}

fn get_token() -> Option<String> {
    fs::read_to_string(get_token_path()).ok().map(|s| s.trim().to_string())
}

fn save_token(token: &str) -> io::Result<()> {
    let path = get_token_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, token)
}

fn clear_token() -> io::Result<()> {
    let path = get_token_path();
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

fn mock_validate_login(username: &str, password: &str) -> Option<String> {
    if username == "admin" && password == "secret" {
        Some("mock_token_abc123xyz".to_string())
    } else {
        None
    }
}

#[derive(Parser, Debug)]
#[command(name = "paas-cli")]
#[command(version = "0.1.0")]
#[command(about = "CLI for rust-paas project")]
struct Args {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Login to the platform
    Login {
        /// Username for authentication
        #[arg(short, long)]
        username: String,
    },
    /// Logout from the platform
    Logout,
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

    match &args.command {
        Some(Commands::Login { username }) => {
            print!("Password: ");
            io::stdout().flush().unwrap();
            let password = read_password().unwrap();
            
            if let Some(token) = mock_validate_login(username, &password) {
                if let Err(e) = save_token(&token) {
                    eprintln!("Failed to save token: {}", e);
                    std::process::exit(1);
                }
                println!("Login successful!");
            } else {
                eprintln!("Login failed: invalid credentials");
                std::process::exit(1);
            }
        }
        Some(Commands::Logout) => {
            if let Err(e) = clear_token() {
                eprintln!("Logout failed: {}", e);
                std::process::exit(1);
            }
            println!("Logout successful!");
        }
        None => {
            println!("paas-cli {}", env!("CARGO_PKG_VERSION"));
        }
    }
}
