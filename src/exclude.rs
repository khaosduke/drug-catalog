use colored::Colorize;
use csv::StringRecord;
use std::collections::HashSet;

pub fn exclude(input_file: &str, exclusions_dir: &str, output_file: &str) 
    -> Result<(), Box<dyn std::error::Error>> {

        let temp_file = "./output/temp.csv";

        //Ensure the exclusions are sane
        check_exclusions(&exclusions_dir)?;
        //Remove all schedule one drugs
        remove_schedule_1(&input_file,&temp_file)?;

        //Extract the exclusions to a hashset
        let excluded_drugs = exclusions_to_hashset(&exclusions_dir)?;
        remove_exclusions(&temp_file,&output_file,excluded_drugs)?;
        
        Ok(())
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
        } else {
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

fn remove_exclusions(input_file: &str, output_file: &str, exclusion_set: std::collections::HashSet<String>) 
    -> Result<(), Box<dyn std::error::Error>> {
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

fn exclusions_to_hashset(exclusions_dir: &str) 
    -> Result<HashSet<String>, Box<dyn std::error::Error>> {
        //Create empty hashset
        let mut exclusions_hashset = HashSet::new();

        let exclusion_files = std::fs::read_dir(exclusions_dir)?;
        for exclusion_file in exclusion_files {
            //Read each csv file
            let csv_reader = csv::Reader::from_path(exclusion_file?.path());
            for row in csv_reader?.records() {
                let record: StringRecord = row?;
                exclusions_hashset.insert(normalize(&record[0]));
            }
        }
        Ok(exclusions_hashset)
    }


fn check_exclusions(exclusions_dir: &str) 
    -> Result<(), Box<dyn std::error::Error>> {
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
        //If not even a file, error out
        if !entry.metadata()?.is_file() {
            return Err("Exclusion file is not a regular file".into());
        }
        check_file_name_format(&file_name)?;
        file_list.push(file_name.clone());
    }

    print_vec_with_newlines(&file_list);
    Ok(())
}

fn check_file_name_format(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let extensions = file_name.split('.').collect::<Vec<&str>>();
    //The file format, if correct is "file.exc.csv" should have minimum 3 elements
    if extensions.len() < 3 {
        return Err("Exclusion file is not properly named".into());
    }
    //Iterate in reverse order to check the last two elements
    for (i, ext) in extensions.iter().rev().enumerate() {
        if i == 0 && *ext != "csv" {
            return Err("Exclusion file is not a CSV file".into());
        }
        if i == 1 && *ext != "exc" {
            return Err("Exclusion file is not properly named".into());
        }
    }
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