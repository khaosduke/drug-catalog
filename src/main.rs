use clap::Parser;
use std::fs;
use colored::Colorize;
use csv::StringRecord;
use std::collections::HashSet;
//use std::path::Path;

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

fn main() {
    let args = Args::parse();
    let temp_file = "./output/temp.csv";

    println!("Input file: {}", args.input);
    println!("Exclusions directory: {}", args.exclusions);
    println!("Output file: {}", args.output);

    if let Err(e) = check_exclusions(&args.exclusions) {
        eprintln!("Error checking exclusions: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = remove_schedule_1(&args.input, &temp_file) {
        eprintln!("Error processing input file: {}", e);
        std::process::exit(1);
    }

    match exclusions_to_list(&args.exclusions) {
        Ok(exclusion_list) => {
            let exclusion_set = list_to_hashset(&exclusion_list);
            //we want the output from removing the schedule one so both
            if let Err(e) = remove_exclusions(&temp_file, &args.output, exclusion_set) {
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

fn remove_exclusions(input_file: &str, output_file: &str, exclusion_set: std::collections::HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
   let mut rdr = csv::Reader::from_path(input_file)?;
   let mut wtr = csv::Writer::from_path(output_file)?;
   
   let mut included_count:i32 = 0;

   //Write the header to the output file
    let headers = rdr.headers()?;
    wtr.write_record(headers)?;

   println!("Removing exclusions from the DEA controlled substances list...");

   for entry in rdr.records() {
       let record = entry?;
        //println!("exclusion c:  {:?}", record[0].to_string());
        //println!("exclusion d:  {:?}", record[0].as_bytes()); 
       
       if !exclusion_set.contains(&normalize(&record[0])) {
            wtr.write_record(&record)?;
            included_count += 1;
        }
    }
    println!("Total records included: {}", included_count);
    println!("----------------------------------------------");
    Ok(())
}

fn exclusions_to_list(exclusions_dir: &str) -> Result<Vec<StringRecord>, Box<dyn std::error::Error>> {
    let mut exclusion_list = Vec::new();
    let exclusion_files = std::fs::read_dir(exclusions_dir)?;

    for entry in exclusion_files {
        //Read each CSV
        let mut rdr = csv::Reader::from_path(entry?.path())?;
        for result in rdr.records() {
            let record: StringRecord = result?;
            //println!("Exclusion record: {:?}", record);
            exclusion_list.push(record);
        }
    }

    println!("Total Exclusion: {}", exclusion_list.len());

    Ok(exclusion_list)
}

fn list_to_hashset(exclusion_list: &Vec<StringRecord>) -> std::collections::HashSet<String> {
    let mut exclusion_set = std::collections::HashSet::new();
    for record in exclusion_list {
        //println!("exclusion a:  {:?}", record[0].to_string());
        //println!("exclusion b:  {:?}", record[0].as_bytes());

        exclusion_set.insert(normalize(&record[0]));
    }

    let debug_string = format!("{:?}", exclusion_set);
    println!("Exclusion set: {}", debug_string);
    exclusion_set
}

fn check_exclusions(exclusions_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    //Checks that the exclusion files exist and are readable. If not, returns an error.
    let exclusion_files = std::fs::read_dir(exclusions_dir)?;
    let mut file_list = Vec::new();
    
    for entry in exclusion_files {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        
        //For macOS
        if file_name == ".DS_Store" {
            continue;
        }
        println!("Checking file: {:?}", file_name);

        file_list.push(file_name.clone());

        let mut extensions = file_name.split('.').collect::<Vec<&str>>();
        //Remove the first element
        extensions.remove(0);

        if !entry.metadata()?.is_file() {
            return Err("Exclusion file is not a regular file".into());
        }
        if (extensions[1] != "csv") || (extensions[0] != "exc" )  {
            return Err("Exclusion files not properly named".into()); 
        }
    }

    print_vec_with_newlines(&file_list);
    Ok(())
}

fn print_vec_with_newlines(vec: &Vec<String>) {
    for item in vec {
        println!("{}", item.green());
    }
}

fn normalize(value: &str) -> String {
    //Normalize but also remove the chemical name from the string. 
    //This is because the chemical name is not always present in the exclusionlist, 
    //And may use symbols not easily represented in a CSV file. 
    // The DEA list uses chemical name sometimes in parenthesis
    value
        .trim()
        .trim_start_matches('\u{feff}')
        .split('(').next()
        .unwrap_or("")
        .to_uppercase()

}