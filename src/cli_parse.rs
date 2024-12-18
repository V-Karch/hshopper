use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None, disable_help_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    ListSupported,
    Search { title: Vec<String> },
    Add { id: u32, name: Vec<String> },
    Download { name: Vec<String> },
    BatchDownload { titles: Vec<String> },
    NetSearch { title: Vec<String> },
}
