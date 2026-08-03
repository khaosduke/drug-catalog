use clap::{Parser, Subcommand};


mod exclude;
use exclude::exclude;

mod map;
use map::map;



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

    /// Look up drugs through RxNorm
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
    }


    Ok(())
}

