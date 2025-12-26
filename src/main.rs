use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod config;
mod platform;
mod templates;

#[derive(Parser)]
#[command(name = "nih-bootstrap")]
#[command(version, about = "Bootstrapper for NIH-plugin development on mac, windows and linux.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new plugin project
    Init {
        /// Project name (kebab-case, snake_case or CamelCase)
        name: String,

        /// Project Path
        #[arg(short, long, default_value = ".")]
        path: String,

        /// Project description
        #[arg(
            short,
            long,
            default_value = "Rust audio plugin project using nih-plug"
        )]
        description: String,

        /// Initialize Git repository
        #[arg(short, long)]
        git: bool,

        /// Set up CI/CD workflows
        #[arg(short, long)]
        ci: bool,

        /// GUI framework to use (iced, egui, etc.)
        #[arg(long, default_value = "iced")]
        gui: String,
    },

    /// Check and install required dependencies
    Deps {
        /// Install missing dependencies automatically
        #[arg(short, long)]
        install: bool,
    },

    /// Add git/CI to an existing project
    Git {
        /// Set up CI/CD workflows
        #[arg(short, long)]
        ci: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {
            name,
            path,
            description,
            git,
            ci,
            gui,
        } => {
            commands::init::run(name, path, description, *git, *ci, gui).await?;
        }
        Commands::Deps { install } => {
            commands::dependencies::check_and_install(*install).await?;
        }
        Commands::Git { ci } => {
            commands::git::setup(*ci).await?;
        }
    }

    Ok(())
}
