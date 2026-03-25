mod commands;

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use commands::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let _ = rustls::crypto::ring::default_provider().install_default();
    let cli = Cli::parse();

    match cli.command {
        Commands::Sign(args) => commands::sign::execute(args).await?,
        Commands::Account(args) => commands::account::execute(args).await?,
        Commands::Device(args) => commands::device::execute(args).await?,
        Commands::Refresh(args) => commands::refresh::execute(args).await?,
        Commands::Inspect(args) => commands::macho::execute(args).await?,
    }

    Ok(())
}

pub fn get_data_path() -> PathBuf {
    let base = if cfg!(windows) {
        env::var("APPDATA").unwrap()
    } else {
        env::var("HOME").unwrap() + "/.config"
    };

    let dir = resolve_data_path(Path::new(&base));

    fs::create_dir_all(&dir).ok();

    dir
}

fn resolve_data_path(base: &Path) -> PathBuf {
    let preferred = base.join("Impactor");
    let legacy = base.join("PlumeImpactor");

    if preferred.exists() || !legacy.exists() {
        preferred
    } else {
        legacy
    }
}
