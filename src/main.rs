use clap::{Parser, Subcommand};


mod exclude;
use exclude::exclude;

mod map;
use map::map;

mod concept;
use concept::concepts;

mod parser;
use parser::parse;

mod curate;
use curate::curate;

use std::fs;
use std::path::Path;



/// Strips down a DEA controlled substances list to only the relevant columns and removes any entries that are in the exclusion list.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Remove excluded drugs from the DEA list
    Filter {
        /// Input CSV file containing the DEA controlled substances list
        #[arg(
            short ='i',
            long,
            default_value = "./input/dea_controlled_substances.csv"
        )]
        input: String,
        /// Directory containing CSV files with drugs to exclude
        #[arg(
            short ='e',
            long,
            default_value = "./exclusion_lists"
        )]
        exclusions: String,
        /// Output CSV file for the filtered DEA list
        #[arg(
            short ='o',
            long,
            default_value = "./output/output.csv"
        )]
        output: String,
    },

    /// Look up drugs through RxNorm, get RXCUI
    Map {
        #[arg(
            short ='i',
            long,
            default_value = "./output/output.csv"
        )]
        input: String,

        #[arg(
            short ='o',
            long,
            default_value = "./output/rxnorm_catalog.csv"
        )]
        output: String,
    },

    /// Get all available drug concepts for a given drug name. 
    /// Takes in a CSV of drugs with RXCUI
    /// Outputs a directory with a CSV of all concepts for each drug name
    /// Eg. output/drug_name_concepts/{rxcui}_drug-name.csv
    Concepts {
        #[arg(
            short ='i',
            long,
            default_value = "./output/rxnorm_catalog.csv"
        )]
        input: String,              

        #[arg(
            short ='o',
            long,
            default_value = "./output/drug_name_concepts"
        )]
        output: String,
    },

    ParseDrug {
        #[arg(
            short ='i',
            long,
            default_value = "./output/rxnorm_catalog.csv"
        )]
        input: String,              

        #[arg(
            short ='o',
            long,
            default_value = "./output/parsed_drugs.csv"
        )]
        output: String,
    },

    Curate {
        #[arg(
            short ='i',
            long,
            default_value = "./output/parsed_drugs.csv"
        )]
        input: String,              

        #[arg(
            short ='o',
            long,
            default_value = "./output/curated_drugs.csv"
        )]
        output: String,
    },
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
   
    match args.command {
        
        Commands::Filter {
            input,
            exclusions,
            output,
        } => {
            exclude(&input, &exclusions, &output)?;
        }

        Commands::Map { input, output } => {
            println!("Mapping {input} to {output}");
            map(&input, &output).await?;
        }

        Commands::Concepts { input, output } => {
            println!("Getting concepts for {input} and writing to {output}");
            concepts(&input, &output).await?;   
        }

        Commands::ParseDrug { input, output } => {
            println!("Parsing drugs from {input} and writing to {output}");
            //let i = "output/drug_name_concepts/6470_lorazepam.csv";
            //let f ="output/drug_name_concepts/4337_fentanyl.csv";
            let p = Path::new("output/drug_name_concepts");
            process_directory(&p).await?;
            
        }

        Commands::Curate { input, output }  => {
            println!("Curating drugs from {input} and writing to {output}");
            let p = Path::new("output/parsed_drugs.csv");
            curate(&input, &output).await?;
        }
    }


    Ok(())
}

async fn parse_drug_file(
    input_file: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    //Open CSV file and parse each drug name, writing the results to a new CSV file
    let mut rdr = csv::Reader::from_path(input_file)?;
    for entry in rdr.records() {
        let record = entry?;
        let drug_name = &record[1];
        let rxcui = &record[0];
        let tty = &record[3];
        println!("Parsing drug: {drug_name} with RXCUI: {rxcui}");
        let parsed = parse(rxcui,tty,drug_name)?;
        println!("Got: {:?}", parsed);
    }
    
    
    Ok(())
}

async fn process_directory(directory: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            println!("Processing: {}", path.display());
            parse_drug_file(&path).await;
        }
    }

    Ok(())
}