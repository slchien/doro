use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "doro",
    author = "Doro Contributors",
    version = doro_core::VERSION,
    about = "AI agent router for MCP"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Serve {
        #[arg(short, long, default_value = "doro.json")]
        config: String,

        #[arg(short, long)]
        profile: Option<String>,
    },
    Profiles,
    Vault,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Serve { config, profile }) => {
            tracing::info!(config = %config, profile = ?profile, "Starting doro router");
            println!("Doro router starting with config: {}", config);
        }
        Some(Commands::Profiles) => {
            println!("Doro profiles management");
        }
        Some(Commands::Vault) => {
            println!("Doro vault management");
        }
        None => {
            println!(
                "Doro v{} - AI agent router for MCP. Use --help for usage.",
                doro_core::VERSION
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse_version() {
        let cli = Cli::try_parse_from(["doro"]).unwrap();
        assert!(cli.command.is_none());
    }

    #[test]
    fn test_cli_parse_serve() {
        let cli = Cli::try_parse_from([
            "doro",
            "serve",
            "--config",
            "custom.json",
            "--profile",
            "ci",
        ])
        .unwrap();
        match cli.command {
            Some(Commands::Serve { config, profile }) => {
                assert_eq!(config, "custom.json");
                assert_eq!(profile, Some("ci".to_string()));
            }
            _ => panic!("Expected Serve command"),
        }
    }
}
