use rxnorm_api::RxNormApi;
use serde_json::{Value};
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt;

#[derive(PartialEq)]
enum ResultType {
    ExactMatch,
    ApproximateMatch,
    NoMatch,
}

impl Display for ResultType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResultType::ExactMatch => write!(f, "Exact Match"),
            ResultType::ApproximateMatch => write!(f, "Approximate Match"),
            ResultType::NoMatch => write!(f, "No Match"),
        }
    }
}
pub async fn map(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    dea_to_rxcui(input, output).await?;
    Ok(())
}

// Gets the RXCUI for all the DEA names in the input CSV file and writes them to the output CSV file
async fn dea_to_rxcui(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut rdr = csv::Reader::from_path(input)?;
    let mut wtr = csv::Writer::from_path(output)?;

    let rxnorm = RxNormApi::new()?;

    //Std options for the find rxcui, exact normalized search
    let  exact_options = HashMap::from([
        ("format", "json"),
        ("search", "2"),
    ]);

    let approx_options = HashMap::from([
        ("format", "json"),
        ("search", "9"),
    ]);

    //Write the header row to the output file
    wtr.write_record(&["DEA Name", "RX CUI", "Exact Match"])?;

    for result in rdr.records() {
        let record = result?;
        let dea_name = first_token_normalized(&record[0]);
     
        let (json,result_type): (Value, ResultType) = loop {
            let response = rxnorm.find_rxcui_by_string(
                &dea_name,
                &exact_options).await?;

            let response_body = response.text().await?;
            let json_response: Value = serde_json::from_str(&response_body)?;
            
            //Check the response, if its exact we are done
            match check_exact_match(&json_response) {
                ResultType::ExactMatch => break (json_response, ResultType::ExactMatch),
                ResultType::ApproximateMatch => break (json_response, ResultType::ApproximateMatch),
                ResultType::NoMatch => {
                    //If not do an approximate search
                    let response = rxnorm.find_rxcui_by_string(
                                    &dea_name,
                                    &approx_options).await?;

                    let response_body = response.text().await?;
                    let json_response: Value = serde_json::from_str(&response_body)?;
                    break (json_response, ResultType::ApproximateMatch);    
                },
            }
        };

        let output_record = [
            &dea_name,
            &get_rxcui(&json).unwrap_or_else(|| "N/A".to_string()),
            &result_type.to_string()
        ];


        println!("Got: {:?}", json["idGroup"]["rxnormId"]);
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

fn check_exact_match(json_response: &Value) -> ResultType {
    //Get the array
    if json_response["idGroup"]["rxnormId"].is_null() {
        return ResultType::NoMatch;
    }

    let rxnorm_ids = json_response["idGroup"]["rxnormId"].as_array().unwrap();
    //If we get two ids we dont have exact match 
    if rxnorm_ids.len() > 1 {
        return ResultType::ApproximateMatch;
    }
    ResultType::ExactMatch
}

//Just in case we get a drug name with a chemical name following
fn first_token_normalized(input: &str) -> String {
    let first_token = input.split("(").next().unwrap_or("");
    first_token.to_lowercase()
}