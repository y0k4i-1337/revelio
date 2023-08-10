use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get resources in a tenant
    Get { resource: Resource },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Resource {
    /// Get the profile of the current user
    Me,
    /// Get the list of users in the tenant
    Users,
    /// Get the list of groups in the tenant
    Groups,
}
