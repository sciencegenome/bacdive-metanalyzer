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
    ID {
     /// please provide the path to the bacdive file
      bacdive_analyzer: String,
     /// please provide the specific id that you want to look
      id: String,
    },
    /// prepare the data for the backhand api integration
    Species {
      /// please provide the path for the bacdive file
      bacdive_analyzer: String,
      /// please provide the specific species that you want to look
      species: String,
    },
    /// please provide the id of the species that you want to look,
    Designation {
        /// please provide the path to the bacdive file
        bacdive_analyzer: String,
        /// please provide the designation header that you want to search
        designation_header: String,
    },
    /// please provide the species that need to be searched.
    Strain {
        /// please provide the path to the bacdive file
        bacdive_analyzer: String,
        /// please provide the strain number that you want to look for.
        strain: String,
    },
    /// please provide the type for the bacdive that you want to look
    StrainType {
        /// please provide the path to the bacdive file
        bacdive_analyzer: String,
        /// type strain that you want to look for in the bacdive.
        strain_type: String,
    },
    /// please provide the bacdiveid for the link to prepare
    BacdiveWeb {
       /// please provide the path to the bacdive file
       bacdive_analyzer: String,
       /// this will take a text file with the ids to generate the bacdive links.
       csv: Option<String>
    }

}
