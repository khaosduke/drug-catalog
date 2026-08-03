use rxnorm_api::RxNormApi;
use serde_json::{Value};
use std::collections::HashMap;


pub async fn map(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    dea_to_rsxui(input, output).await?;
    Ok(())
}

// Gets the RXCUI for all the DEA names in the input CSV file and writes them to the output CSV file
async fn dea_to_rsxui(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut rdr = csv::Reader::from_path(input)?;
    let mut wtr = csv::Writer::from_path(output)?;

    let rxnorm = RxNormApi::new()?;
    //Std options for the find rxcui, exact normalized search
    let mut findrxcui_options = HashMap::from([
        ("format", "json"),
        ("search", "2"),
    ]);

    //Write the header row to the output file
    wtr.write_record(&["DEA Name", "RX CUI", "Exact Match"])?;

    for result in rdr.records() {
        let record = result?;
        let dea_name = first_token_normalized(&record[0]);

        let response = rxnorm.find_rxcui_by_string(
                &first_token_normalized(&dea_name),
                &findrxcui_options).await?;

        let response_body = response.text().await?;
        let json_response: Value = serde_json::from_str(&response_body)?;

        let output_record = [
            &dea_name,
            &get_rxcui(&json_response).unwrap_or_else(|| "N/A".to_string()),
            &check_exact_match(&json_response).to_string()
        ];


        println!("Got: {:?}", json_response["idGroup"]["rxnormId"]);
        // Write the filtered record to the output file
        wtr.write_record(&output_record)?;
    }

    wtr.flush()?;
    Ok(())
}

fn get_rxcui(json_response: &Value) -> Option<String> {
    if json_response["idGroup"]["rxnormId"].is_null() {
        return None;
    }

    let rxnorm_ids = json_response["idGroup"]["rxnormId"].as_array().unwrap();
    if rxnorm_ids.is_empty() {
        return None;
    }

    Some(rxnorm_ids[0].as_str().unwrap_or("").to_string())
}

fn check_exact_match(json_response: &Value) -> bool {
    //Get the array
    if json_response["idGroup"]["rxnormId"].is_null() {
        return false;
    }

    let rxnorm_ids = json_response["idGroup"]["rxnormId"].as_array().unwrap();
    //If we get two ideas we dont have exact match 
    if rxnorm_ids.len() > 1 {
        return false;
    }
    true
}

//Just in case we get a drug name with a chemical name following
fn first_token_normalized(input: &str) -> String {
    let first_token = input.split_whitespace().next().unwrap_or("");
    first_token.to_lowercase()
}