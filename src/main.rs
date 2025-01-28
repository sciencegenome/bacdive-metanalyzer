mod args;
mod genome;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::genome::Bacterialphyla;
use crate::genome::GenomeReferences;
use clap::Parser;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use select::document;
use std::error::Error;

/*
*Author Gaurav Sablok
*Universitat Potsdam and SLB Potsdam
*Date 2024-1-28


bacdive meta-analyzer: this analyzes the meta data from the bacdive associated
with that specific strain and gives you the following information on that
specific strain:
Strain information:
Phylum, Class Order, Family, Genus, Species
NCBI-taxID, ncbi-GenBank, ncbi-ENA, Literature and References.

*/

fn main() {
    let bacdiveargs = CommandParse::parse();
    match &bacdiveargs.command {
        Commands::Strain { strain } => {
            let commandout = bacdive_search(strain).unwrap();
            println!(
                "The associated information with this strain is: {:?}",
                commandout
            );
        }
    }
}

fn bacdive_search(strain: &str) -> Result<String, Box<dyn Error>> {
    let weblink = String::from("https://bacdive.dsmz.de/strain");
    let id = strain;
    let finaldownload: String = format!("{}{}", weblink, id);

    let datalink = reqwest::blocking::get(finaldownload)
        .unwrap()
        .text()
        .unwrap();

    let mut bacterialphylainformation: Vec<String> = Vec::new();
    let mut bacterialreferenceinformation: Vec<String> = Vec::new();
    let mut bacterialsequenceinformation: Vec<String> = Vec::new();
    let mut referencesvector: Vec<String> = Vec::new();

    let parsedstrainpage = Html::parse_document(&datalink);
    let phylaselect = scraper::Selector::parse(". valigntop paddingright <a").unwrap();
    let seqlink = scraper::Selector::parse(". seq-links r").unwrap();
    let ncbilink = scraper::Selector::parse(". r ncbi-tax-link").unwrap();
    let referencelink = scraper::Selector::parse(". resultdetail_reference_refid").unwrap();

    for phyla in parsedstrainpage.select(&phylaselect) {
        if let Some(href) = phyla.value().attr("href") {
            bacterialphylainformation.push(href.to_string());
        }
    }

    for seq in parsedstrainpage.select(&seqlink) {
        if let Some(href) = seq.value().attr("href") {
            bacterialsequenceinformation.push(href.to_string());
        }
    }

    for id in parsedstrainpage.select(&ncbilink) {
        if let Some(href) = id.value().attr("href") {
            bacterialreferenceinformation.push(href.to_string());
        }
    }

    for iterref in parsedstrainpage.select(&referencelink) {
        if let Some(href) = iterref.value().attr("href") {
            referencesvector.push(href.to_string());
        }
    }

    let phylaselect = parsedstrainpage
        .select(&referencelink)
        .next()
        .and_then(|a| a.value().attr("href"))
        .map(str::to_owned)
        .into_iter()
        .collect::<Vec<_>>();

    let seqselect = parsedstrainpage
        .select(&seqlink)
        .next()
        .and_then(|a| a.value().attr("href"))
        .map(str::to_owned)
        .into_iter()
        .collect::<Vec<_>>();

    let ncbiselect = parsedstrainpage
        .select(&ncbilink)
        .next()
        .and_then(|a| a.value().attr("href"))
        .map(str::to_owned)
        .into_iter()
        .collect::<Vec<_>>();

    let referenceselect = parsedstrainpage
        .select(&referencelink)
        .next()
        .and_then(|a| a.value().attr("href"))
        .map(str::to_owned)
        .into_iter()
        .collect::<Vec<_>>();

    Ok("The results for the following strain are as follow".to_string())
}
