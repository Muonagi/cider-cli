use clap::{Parser, Subcommand};

pub mod account;
pub mod device;
pub mod macho;
pub mod refresh;
pub mod session;
pub mod sign;

#[derive(Debug, Parser)]
#[command(
    name = "impactor",
    author,
    version,
    about = "Interactive CLI for iOS signing, install, accounts, and refresh",
    disable_help_subcommand = true,
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Sign an iOS app bundle with certificate and provisioning profile
    Sign(sign::SignArgs),
    /// Manage Apple Developer account authentication
    Account(account::AccountArgs),
    /// Device management commands
    Device(device::DeviceArgs),
    /// Manage refresh registrations and run refresh flows
    Refresh(refresh::RefreshArgs),
    /// Inspect Mach-O binaries
    Inspect(macho::MachArgs),
}
