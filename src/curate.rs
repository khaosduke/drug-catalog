use parser::parse;
use std::fs;
use std::path::Path;

pub async fn curate(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let p = Path::new(input);
    process_directory(&p).await?;
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