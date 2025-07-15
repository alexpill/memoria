mod cli;

use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use memoria::MemoriaConfig;

use cli::{Cli, Commands, ConfigCommands};

fn main() -> Result<()> {
    // Initialize environment
    dotenv().ok();
    env_logger::init();

    // Ensure config file exists and load configuration
    MemoriaConfig::ensure_config_exists()?;
    let config = MemoriaConfig::load()?;

    // Parse command line arguments
    let cli = Cli::parse();

    // Dispatch to appropriate handler
    match cli.command {
        Commands::List => cli::handle_list(&config),
        Commands::Create { title } => cli::handle_create(&title, &config),
        Commands::Init { title } => cli::handle_init(&title, &config),
        Commands::Config { config_command } => match config_command {
            ConfigCommands::Show => cli::handle_config_show(&config),
            ConfigCommands::Edit => cli::handle_config_edit(&config),
            ConfigCommands::Set { key, value } => cli::handle_config_set(&key, &value),
            ConfigCommands::Get { key } => cli::handle_config_get(&key, &config),
            ConfigCommands::Reset => cli::handle_config_reset(),
        },
    }
}
