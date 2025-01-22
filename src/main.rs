mod args;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

/*
*Author Gaurav Sablok
*Universitat Potsdam and SLB Potsdam
*Date 2024-1-22
bacdive meta-analyzer: this analyzes the meta data from the bacdive
along with the sequence analysis embedded within the bacdive-metaanalyzer.

*/

fn main() {
    let bacdiveargs = CommandParse::parse();
    match &bacdiveargs.command {
        Commands::Strain {
            bacdive_analyzer,
            strain,
        } => {
            let commandoutput = bacdive_search(bacdive_analyzer, strain).unwrap();
        }
    }
}
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct BacdiveSearchPattern {
    pub id: String,
    pub species: String,
    pub designation_header: String,
    pub strain_number: String,
    pub type_strain: String,
}


fn bacdive_search(path: &str, strain: &str) -> Result<String, Box< dyn Error>>{

  Ok("The results for the following strain are as follow".to_string())

}
