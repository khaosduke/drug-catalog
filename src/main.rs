use clap::Parser;
use clap::{Command, arg};
use std::collections::HashMap;


mod exclude;
use exclude::exclude;

use rxnorm_api::RxNormApi;


/// Strips down a DEA controlled substances list to only the relevant columns and removes any entries that are in the exclusion list.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Main DEA full list, in CSV format
    #[arg(short, long, default_value = "./input/dea_controlled_substances.csv")]
    input: String,

    /// Exclusion list directory
    #[arg(short, long, default_value = "./exclusion_lists")]
    exclusions: String,

    /// Output directory
    #[arg(short, long, default_value = "./output/output.csv")]
    output: String,

}

fn cli() -> Command {
    Command::new("drugcat")
        .about("Used to filter out drug lists from the DEA using RxNorm as the reference database")
        .subcommand_required(true)
        .subcommand(
            Command::new("exclude")
            .about("Removes drugs from the standard DEA list based on a set of exclusion lists")
        )
        .subcommand(
            Command::new("lookup")
            .about("Look up drugs from input csv with RxNorm")
        )
        
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    

    println!("Input file: {}", args.input);
    println!("Exclusions directory: {}", args.exclusions);
    println!("Output file: {}", args.output);

    let rxnorm = RxNormApi::new()?;

    let drug_rxcui = "4337";//Fentanyl

    let ops = HashMap::from([
        ("format","json"),
        ("tty","SCD SBD SCDG SBDG")
    ]);

    //let response = rxnorm.get(drug_related_by_type_function,&relatedbytype_ops).await?;
    let response = rxnorm.get_related_by_type(drug_rxcui, &ops).await?;
    println!("Got: {:?}",response.text().await?);




    //let _ = exclude(&args.input,&args.exclusions,&args.output)?;

    

    Ok(())
}

