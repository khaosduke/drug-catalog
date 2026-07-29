use clap::Parser;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    

    println!("Input file: {}", args.input);
    println!("Exclusions directory: {}", args.exclusions);
    println!("Output file: {}", args.output);

    let _ = exclude(&args.input,&args.exclusions,&args.output)?;

    Ok(())
}

