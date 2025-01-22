use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "bacdive",
    version = "1.0",
    about = "prepairing the json for the bacdive"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// search for the specific species in the bacdive to get their strain and associated
    /// information
    Strain {
     /// please provide the path to the bacdive file
      bacdive_analyzer: String,
     /// please provide the specific id that you want to look
      strain: String,
    }
}
