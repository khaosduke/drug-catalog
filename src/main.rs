use clap::{Parser, Subcommand};
use std::collections::HashMap;


mod exclude;
use exclude::exclude;

use rxnorm_api::RxNormApi;


/// Strips down a DEA controlled substances list to only the relevant columns and removes any entries that are in the exclusion list.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Remove excluded drugs from the DEA list
    Filter {
        #[arg(
            short,
            long,
            default_value = "./input/dea_controlled_substances.csv"
        )]
        input: String,

        #[arg(
            short,
            long,
            default_value = "./exclusion_lists"
        )]
        exclusions: String,

        #[arg(
            short,
            long,
            default_value = "./output/output.csv"
        )]
        output: String,
    },

    /// Look up drugs through RxNorm
    Map {
        #[arg(
            short,
            long,
            default_value = "./output/output.csv"
        )]
        input: String,

        #[arg(
            short,
            long,
            default_value = "./output/rxnorm_catalog.csv"
        )]
        output: String,
    },
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    

    //println!("Input file: {}", args.input);
    //println!("Exclusions directory: {}", args.exclusions);
    //println!("Output file: {}", args.output);

    //let rxnorm = RxNormApi::new()?;

    //let drug_rxcui = "4337";//Fentanyl

    //let ops = HashMap::from([
    //    ("format","json"),
    //    ("tty","SCD SBD SCDG SBDG")
    //]);

    //let response = rxnorm.get(drug_related_by_type_function,&relatedbytype_ops).await?;
    //let response = rxnorm.get_related_by_type(drug_rxcui, &ops).await?;
    //println!("Got: {:?}",response.text().await?);




    //let _ = exclude(&args.input,&args.exclusions,&args.output)?;

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

            let rxnorm = RxNormApi::new()?;

            let options = HashMap::from([
                ("format", "json"),
                ("tty", "SCD SBD SCDG SBDG"),
            ]);

            let response = rxnorm
                .get_related_by_type("4337", &options)
                .await?;

            println!("{}", response.text().await?);
        }
    }


    Ok(())
}

