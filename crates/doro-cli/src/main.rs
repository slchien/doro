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
    /// Manage profiles and tool policies
    Profiles {
        #[arg(short, long, default_value = "doro.json")]
        config: String,
    },
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
        Some(Commands::Profiles { config }) => {
            let contents = std::fs::read_to_string(&config)?;
            let profiles = doro_policy::ProfileSet::from_config_json(&contents)?;

            let mut names: Vec<&str> = profiles.names().collect();
            names.sort();

            if names.is_empty() {
                println!("No profiles defined in {}", config);
            } else {
                println!("Profiles in {}:", config);
                for name in names {
                    let profile = profiles.get(name)?;
                    println!(
                        "  {} (default: {:?}, {} rule(s))",
                        name,
                        profile.default_action,
                        profile.rules.len()
                    );
                }
            }
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

    #[test]
    fn test_cli_parse_profiles() {
        let cli = Cli::try_parse_from(["doro", "profiles", "--config", "custom.json"]).unwrap();
        match cli.command {
            Some(Commands::Profiles { config }) => {
                assert_eq!(config, "custom.json");
            }
            _ => panic!("Expected Profiles command"),
        }
    }
}
