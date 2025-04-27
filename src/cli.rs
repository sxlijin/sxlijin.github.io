use anyhow::Result;
use clap::{Parser, Subcommand};
use sam_website::{build_website, start_server};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Build the site
    Build,

    /// Run the development server
    Dev,
}

pub fn parse() -> Cli {
    Cli::parse()
}

// Setup tracing for the application
fn setup_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(true)
                .with_target(false)
                .with_thread_ids(false)
                .with_file(true)
                .with_line_number(true),
        )
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing();
    let cli = parse();
    match cli.command {
        Commands::Build => {
            build_website()?;
            Ok(())
        }
        Commands::Dev => start_server(3000).await,
    }
}
