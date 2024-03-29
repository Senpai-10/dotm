use clap::{Parser, Subcommand};

/// Dotfiles manager
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct DotmArgs {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add dotfile
    Add {
        source: String,
        destination: String,
    },

    /// List all dotfiles
    List {},

    /// remove dotfile
    Remove {
        /// dotfile path or use 'all' to remove all dotfiles
        path: String
    },

    // Backup all dotfiles
    Backup {},
}
