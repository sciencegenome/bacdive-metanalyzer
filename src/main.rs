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
*Date 2024-1-21
bacdive analyzer - this analyzes the bacdive isolates and
provides a approach for the identification and retrieval
of the isolates and also allows for the comparative analysis.
started writing this morning at Universitat Potsdam and finished
by afternoon.

*/

fn main() {
    let bacdiveargs = CommandParse::parse();
    match &bacdiveargs.command {
        Commands::ID {
            bacdive_analyzer,
            id,
        } => {
            let commandoutput = id_search(bacdive_analyzer, id).unwrap();
            for i in commandoutput.iter() {
                println!(
                "The id of the species is:{:?}\nThe species number is {:?}\nThe designation header is: {:?}\nThe strain number is: {:?}\nThe type strain for the sequence is: {:?}",i.id, i.species, i.designation_header, i.strain_number, i.type_strain
            );
            }
        }
        Commands::Species {
            bacdive_analyzer,
            species,
        } => {
            let commandoutput = species_search(bacdive_analyzer, species).unwrap();
            for i in commandoutput.iter() {
                println!(
                "The id of the species is:{:?}\nThe species number is {:?}\nThe designation header is: {:?}\nThe strain number is: {:?}\nThe type strain for the sequence is: {:?}",i.id, i.species, i.designation_header, i.strain_number, i.type_strain
            );
            }
        }
        Commands::Designation {
            bacdive_analyzer,
            designation_header,
        } => {
            let commandoutput = designation_search(bacdive_analyzer, designation_header).unwrap();
            for i in commandoutput.iter() {
                println!(
                "The id of the species is:{:?}\nThe species number is {:?}\nThe designation header is: {:?}\nThe strain number is: {:?}\nThe type strain for the sequence is: {:?}",i.id, i.species, i.designation_header, i.strain_number, i.type_strain
            );
            }
        }
        Commands::Strain {
            bacdive_analyzer,
            strain,
        } => {
            let commandoutput = strain_search(bacdive_analyzer, strain).unwrap();
            for i in commandoutput.iter() {
                println!(
                "The id of the species is:{:?}\nThe species number is {:?}\nThe designation header is: {:?}\nThe strain number is: {:?}\nThe type strain for the sequence is: {:?}",i.id, i.species, i.designation_header, i.strain_number, i.type_strain
            );
            }
        }
        Commands::StrainType {
            bacdive_analyzer,
            strain_type,
        } => {
            let commandoutput = strain_type_search(bacdive_analyzer, strain_type).unwrap();
            for i in commandoutput.iter() {
                println!(
                "The id of the species is:{:?}\nThe species number is {:?}\nThe designation header is: {:?}\nThe strain number is: {:?}\nThe type strain for the sequence is: {:?}",i.id, i.species, i.designation_header, i.strain_number, i.type_strain
            );
            }
        }
        Commands::BacdiveWeb {
            bacdive_analyzer,
            csv,
        } => {
            let commandoutput = bacdiveweb(bacdive_analyzer, csv.as_deref()).unwrap();
            for i in commandoutput.iter() {
                println!(
                "The id of the species is:{:?}\nThe species number is {:?}\nThe designation header is: {:?}\nThe strain number is: {:?}\nThe type strain for the sequence is: {:?}",i.id, i.species, i.designation_header, i.strain_number, i.type_strain
            );
            }
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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct BacdiveSearchSpecies {
    pub id: Vec<String>,
    pub species: String,
    pub designation_header: Vec<String>,
    pub strain_number: Vec<String>,
    pub type_strain: Vec<String>,
}

fn id_search(
    bacdive_analyzer: &str,
    id: &str,
) -> Result<Vec<BacdiveSearchPattern>, Box<dyn Error>> {
    let bacdive_analyzer = File::open(bacdive_analyzer).expect("file not found");
    let bacdive_analyzer_read = BufReader::new(bacdive_analyzer);
    let mut idsearch: Vec<BacdiveSearchPattern> = Vec::new();
    for i in bacdive_analyzer_read.lines() {
        let line = i.expect("line not found");
        let linevec: Vec<_> = line.split(",").collect::<Vec<_>>();
        if linevec[0] == id {
            idsearch.push(BacdiveSearchPattern {
                id: linevec[0].to_string(),
                species: linevec[1].to_string(),
                designation_header: linevec[2].to_string(),
                strain_number: linevec[3].to_string(),
                type_strain: linevec[4].to_string(),
            });
        }
    }
    let bacdiveaddress: String = String::from("http:://bacdive.dsmz.de/strain/");
    let weblink: String = format!("{}{}", bacdiveaddress, id);
    let _ = Command::new("wget")
        .arg("-F")
        .arg(weblink)
        .output()
        .expect("commandfailed");

    Ok(idsearch)
}

fn species_search(
    bacdive_analyzer: &str,
    species: &str,
) -> Result<Vec<BacdiveSearchSpecies>, Box<dyn Error>> {
    let bacdive_analyzer_open = File::open(bacdive_analyzer).expect("file not found");
    let bacdive_analyzer_read = BufReader::new(bacdive_analyzer_open);
    let mut species_search: Vec<BacdiveSearchSpecies> = Vec::new();
    for i in bacdive_analyzer_read.lines() {
        let line = i.expect("line not found");
        let linevec = line.split(",").collect::<Vec<_>>();
        let mut selectedid: Vec<String> = Vec::new();
        let mut header: Vec<String> = Vec::new();
        let mut number: Vec<String> = Vec::new();
        let mut strain: Vec<String> = Vec::new();
        if linevec[1] == species {
            selectedid.push(linevec[0].to_string());
            header.push(linevec[2].to_string());
            number.push(linevec[3].to_string());
            strain.push(linevec[4].to_string());
            species_search.push(BacdiveSearchSpecies {
                id: selectedid,
                species: linevec[1].to_string(),
                designation_header: header,
                strain_number: number,
                type_strain: strain,
            });
        }
    }

    Ok(species_search)
}

fn designation_search(
    bacdive_analyzer: &str,
    designation_header: &str,
) -> Result<Vec<BacdiveSearchPattern>, Box<dyn Error>> {
    let bacdive_analyzer = File::open(bacdive_analyzer).expect("file not found");
    let bacdive_analyzer_read = BufReader::new(bacdive_analyzer);
    let mut designation_header_search: Vec<BacdiveSearchPattern> = Vec::new();
    for i in bacdive_analyzer_read.lines() {
        let line = i.expect("line not found");
        let linevec: Vec<_> = line.split(",").collect::<Vec<_>>();
        if linevec[2] == designation_header {
            designation_header_search.push(BacdiveSearchPattern {
                id: linevec[0].to_string(),
                species: linevec[1].to_string(),
                designation_header: linevec[2].to_string(),
                strain_number: linevec[3].to_string(),
                type_strain: linevec[4].to_string(),
            });
        }
    }

    let mut finalweblink: Vec<String> = Vec::new();
    for i in designation_header_search.iter() {
        let bacdiveaddress: String = String::from("http:://bacdive.dsmz.de/strain/");
        let weblink: String = format!("{}{}", bacdiveaddress, i.id);
        finalweblink.push(weblink)
    }
    for i in finalweblink.iter() {
        let _ = Command::new("wget")
            .arg("-F")
            .arg(i)
            .output()
            .expect("commandfailed");
    }
    Ok(designation_header_search)
}

fn strain_search(
    bacdive_analyzer: &str,
    strain: &str,
) -> Result<Vec<BacdiveSearchPattern>, Box<dyn Error>> {
    let bacdive_analyzer = File::open(bacdive_analyzer).expect("file not found");
    let bacdive_analyzer_read = BufReader::new(bacdive_analyzer);
    let mut strainsearch: Vec<BacdiveSearchPattern> = Vec::new();
    for i in bacdive_analyzer_read.lines() {
        let line = i.expect("line not found");
        let linevec: Vec<_> = line.split(",").collect::<Vec<_>>();
        if linevec[3] == strain {
            strainsearch.push(BacdiveSearchPattern {
                id: linevec[0].to_string(),
                species: linevec[1].to_string(),
                designation_header: linevec[2].to_string(),
                strain_number: linevec[3].to_string(),
                type_strain: linevec[4].to_string(),
            });
        }
    }

    let mut finalweblink: Vec<String> = Vec::new();
    for i in strainsearch.iter() {
        let bacdiveaddress: String = String::from("http:://bacdive.dsmz.de/strain/");
        let weblink: String = format!("{}{}", bacdiveaddress, i.id);
        finalweblink.push(weblink)
    }
    for i in finalweblink.iter() {
        let _ = Command::new("wget")
            .arg("-F")
            .arg(i)
            .output()
            .expect("commandfailed");
    }

    Ok(strainsearch)
}

fn strain_type_search(
    bacdive_analyzer: &str,
    typestrain: &str,
) -> Result<Vec<BacdiveSearchPattern>, Box<dyn Error>> {
    let bacdive_analyzer = File::open(bacdive_analyzer).expect("file not found");
    let bacdive_analyzer_read = BufReader::new(bacdive_analyzer);
    let mut typestrainsearch: Vec<BacdiveSearchPattern> = Vec::new();
    for i in bacdive_analyzer_read.lines() {
        let line = i.expect("line not found");
        let linevec: Vec<_> = line.split(",").collect::<Vec<_>>();
        if linevec[4] == typestrain {
            typestrainsearch.push(BacdiveSearchPattern {
                id: linevec[0].to_string(),
                species: linevec[1].to_string(),
                designation_header: linevec[2].to_string(),
                strain_number: linevec[3].to_string(),
                type_strain: linevec[4].to_string(),
            });
        }
    }

    let mut finalweblink: Vec<String> = Vec::new();
    for i in typestrainsearch.iter() {
        let bacdiveaddress: String = String::from("http:://bacdive.dsmz.de/strain/");
        let weblink: String = format!("{}{}", bacdiveaddress, i.id);
        finalweblink.push(weblink)
    }
    for i in finalweblink.iter() {
        let _ = Command::new("wget")
            .arg("-F")
            .arg(i)
            .output()
            .expect("commandfailed");
    }

    Ok(typestrainsearch)
}

fn bacdiveweb(
    bacdive_analyzer: &str,
    csv: Option<&str>,
) -> Result<Vec<BacdiveSearchPattern>, Box<dyn Error>> {
    let mut csvvec: Vec<String> = Vec::new();
    let mut inputsearchcsv: Vec<BacdiveSearchPattern> = Vec::new();
    if csv.is_some() {
        let csv = csv.unwrap();
        let csvopen = File::open(csv).expect("file not found");
        let csvread = BufReader::new(csvopen);
        for i in csvread.lines() {
            let line = i.expect("line not found");
            let linevec = line
                .trim()
                .split(",")
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();
            csvvec.push(linevec[0].to_string());
        }
        let bacfileopen = File::open(bacdive_analyzer).expect("file not found");
        let bacfileread = BufReader::new(bacfileopen);
        let mut bacfileinput: Vec<BacdiveSearchPattern> = Vec::new();
        for i in bacfileread.lines() {
            let line = i.expect("line not found");
            let linevec: Vec<_> = line.split(",").collect::<Vec<_>>();
            bacfileinput.push(BacdiveSearchPattern {
                id: linevec[0].to_string(),
                species: linevec[1].to_string(),
                designation_header: linevec[2].to_string(),
                strain_number: linevec[3].to_string(),
                type_strain: linevec[4].to_string(),
            });
        }
        for i in bacfileinput.iter() {
            for j in csvvec.iter() {
                if i.id == *j {
                    inputsearchcsv.push(BacdiveSearchPattern {
                        id: i.id.clone(),
                        species: i.species.clone(),
                        designation_header: i.designation_header.clone(),
                        strain_number: i.strain_number.clone(),
                        type_strain: i.type_strain.clone(),
                    });
                }
            }
        }
    }
    let mut bacdiveweblinks: Vec<String> = Vec::new();
    for i in csvvec.iter() {
        let bacdiveaddress: String = String::from("http:://bacdive.dsmz.de/strain/");
        let weblink: String = format!("{}{}", bacdiveaddress, i);
        bacdiveweblinks.push(weblink);
    }

    for i in bacdiveweblinks.iter() {
        let _ = Command::new("wget")
            .arg("-F")
            .arg(i)
            .output()
            .expect("commandfailed");
    }

    Ok(inputsearchcsv)
}
