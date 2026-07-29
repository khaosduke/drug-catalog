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

    ///Temp file
    #[arg(short,long, default_value = "./output/temp.csv")]
    temp: String,

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    println!("Using...");

    println!("Input file: {}", args.input);
    println!("Exclusions directory: {}", args.exclusions);
    println!("Output file: {}", args.output);
    println!("Temp file: {}",args.temp);
    println!("----------------------------------------------");

    let _ = exclude(&args.input,&args.exclusions,&args.output)?;

    if let Err(e) = remove_schedule_1(&args.input, &args.temp) {
        eprintln!("Error processing input file: {}", e);
        std::process::exit(1);
    }

    match exclusions_to_list(&args.exclusions) {
        Ok(exclusion_list) => {
            let exclusion_set = list_to_hashset(&exclusion_list);
            //we want the output from removing the schedule one so both
            if let Err(e) = remove_exclusions(&args.temp, &args.output, exclusion_set) {
                eprintln!("Error removing exclusions from exclusion filest: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error processing exclusions to a vector: {}", e);
            std::process::exit(1);
        }   
    }

   
}

fn remove_schedule_1(input_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    //This is guaranteed read only
    let mut input_rdr = match csv::Reader::from_path(input_file) {
        Ok(rdr) => rdr,
        Err(e) => return Err(format!("Error reading input file: {}", e).into()),
    };
    //In properly formed CSV the third column is the schedule
    //so we can filter out any rows with "I" in that column.
    let mut output_file = match csv::Writer::from_path(output_file) {
        Ok(writer) => writer,
        Err(e) => return Err(format!("Error creating output file: {}", e).into()),
    };

    //Write the header to the output file
    let headers = input_rdr.headers()?;
    output_file.write_record(headers)?;

    let mut excluded_count:i32 = 0;
    let mut total_count:i32 = 0;

    for entry in input_rdr.records() {
        total_count += 1;
        let record = entry?;
        if record.get(2) != Some("I") {
            output_file.write_record(&record)?;
            excluded_count += 1;
        }
    }

    println!("Removing Schedule I substances from the DEA controlled substances list...");
    println!("Total records processed: {}", total_count.to_string().green());
    println!("Total records excluded: {}", excluded_count.to_string().red());
    println!("Total records included: {}", (total_count - excluded_count).to_string().blue());
    println!("----------------------------------------------");
    

    Ok(())
}

