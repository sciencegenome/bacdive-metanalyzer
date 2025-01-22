mod args;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
use reqwest::*;
use scraper::selector;
use select::*;
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
Give a bacdive file from the URL https://bacdive.dsmz.de/advsearch and
then search the strain and it will connect to the bacdive automatically
and will give you the following information:

Strain information:
Phylum, Class Order, Family, Genus, Species
NCBI-taxID, ncbi-GenBank, ncbi-ENA, Literature and References.

Wrote at SLB Potsdam.
*/

fn main() {
    let bacdiveargs = CommandParse::parse();
    match &bacdiveargs.command {
        Commands::Strain {
            bacdive_analyzer,
            strain,
        } => {
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct SeqNCBI {
    sequence: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct SpeciesInformation {
    domain: String,
    phylum: String,
    class: String,
    order: String,
    family: String,
    genus: String,
    species: String,
    scientificname: String,
}

fn bacdive_search(path: &str, strain: &str) -> Result<String, Box<dyn Error>> {
    let bacdiveopen = File::open(path).expect("file not found");
    let bacdiveread = BufReader::new(bacdiveopen);
    let mut bacdivestore: Vec<BacdiveSearchPattern> = Vec::new();
    for i in bacdiveread.lines() {
        let line = i.expect("line not found");
        let linevec: Vec<_> = line.split(",").collect::<Vec<_>>();
        bacdivestore.push(BacdiveSearchPattern {
            id: linevec[0].to_string(),
            species: linevec[1].to_string(),
            designation_header: linevec[2].to_string(),
            strain_number: linevec[3].to_string(),
            type_strain: linevec[4].to_string(),
        })
    }

    let mut finaldownload_url: Vec<String> = Vec::new();
    for i in bacdivestore.iter() {
        if i.id == strain {
            let weblink = String::from("https://bacdive.dsmz.de/strain");
            let id = strain;
            let finaldownload: String = format!("{}{}", weblink, id);
            finaldownload_url.push(finaldownload);
        }
    }

    /*
     * sorting the species and the ncbi link
     * */

    for i in finaldownload_url.iter() {
        let datalink = reqwest::blocking::get(i);
        let web = datalink.unwrap().text().unwrap();
        let selectregion_seqlinks = scraper::Selector::parse(". seq-links r").unwrap();
        let selectregions_ncbi_links = scraper::Selector::parse(". r ncbi-tax-link").unwrap();
        let new_seqlinks = document.select(&selectregion_seqlinks);
        let new_ncbi_links = document.select(&selectregions_ncbi_links);
        let seqlinks_vector: Vec<_> = new_seqlinks.text().collect::<Vec<_>>();
        let ncbi_vector: Vec<_> = new_ncbi_links.text().collect::<Vec<_>>();
    }

    /*  sorting the species information:
     *
     * */

    let mut finaldataspecies: Vec<SpeciesInformation> = Vec::new();
    for i in finaldownload_url.iter() {
        let datalink = reqwest::blocking::get(i);
        let weblink = response.unwrap().text().unwrap();
        let specieslink = scraper::Selector::parse(". valigntop paddingright").unwrap();
        let speciesselected = document.select(&specieslink);
        let speciestext: Vec<_> = speciesselected.text().collect::<Vec<_>>();
        for i in speciestext.iter() {
            finaldataspecies.push(SpeciesInformation {
                domain: speciestext[0],
                phylum: speciestext[1],
                class: speciestext[2],
                order: speciestext[3],
                family: speciestext[4],
                genus: speciestext[5],
                species: speciestext[6],
                scientificname: speciestext[7],
            });
        }
    }


    #[derive(Debug, Clone, PartialOrd, PartialEq)]
    struct References {
        referencesection: String,
        referencelink: String,
    }

    let mut referencesvector: Vec<References> = Vec::new();
    for i in finaldownload_url.iter() {
        let datalink = reqwest::blocking::get(i);
        let weblink = datalink.unwrap().text().unwrap();
        let referencelink = scraper::Selector::parse(". resultdetail_reference_refid").unwrap();
        let referencetext = document
            .select(&referencelink)
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned)
            .collect::<Vec<_>>();
    }

    Ok("The results for the following strain are as follow".to_string())
}
