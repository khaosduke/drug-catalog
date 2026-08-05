use rxnorm_api::RxNormApi;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sanitize_filename::sanitize;

//Since we know the structure of the output json we type it
#[derive(Serialize, Deserialize, Debug)]
struct RxNormConceptsResultJson {
    relatedGroup: RelatedGroup,
}

#[derive(Serialize, Deserialize, Debug)]
struct RelatedGroup {
    rxcui: Option<String>,
    conceptGroup: Vec<ConceptGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConceptGroup {
    tty: String,
    conceptProperties: Option<Vec<ConceptProperty>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConceptProperty {
    rxcui:Option<String>,
    name:Option<String>,
    synonym:Option<String>,
    tty:Option<String>,
    language:Option<String>,
    suppress:Option<String>,
    umlscui:Option<String>,
    psn:Option<String>
}


pub async fn concepts(input: &str, output_dir: &str) 
            -> Result<(), Box<dyn std::error::Error>> {
    
    // Create new directory for the output files
    std::fs::create_dir_all(output_dir)?;
    // Create a new CSV reader for the input file
    let mut drug_list_reader = csv::Reader::from_path(input)?;
    // Create a new RxNormApi instance
    let rxnorm = RxNormApi::new()?;
    //Options for the getRelatedByType function
    let options = HashMap::from([
        ("format", "json"),
        ("tty", "SCD SBD GPCK BPCK"),
    ]);            

    for entry in drug_list_reader.records() {
        let record = entry?;            
        let dea_name = record[0].to_string();
        let rxcui = record[1].to_string();
        let response = 
                rxnorm.get_related_by_type(&rxcui, &options).await?;
        let concepts_body = response.text().await?;        
        let concepts_json: RxNormConceptsResultJson = 
                serde_json::from_str(&concepts_body)?;
                
        println!("Got concepts for {}: {:?}", dea_name, concepts_json);              
        //Extract a header from the concepts json
        //let header = extract_header_from_concepts_json(&concepts);
        let header = vec![
            "rxcui", 
            "name", 
            "synonym", 
            "tty", 
            "language", 
            "suppress", 
            "umlscui", 
            "psn"
        ];

        //Create a new CSV writer for the output file
        let output_file_path = format!("{}/{}_{}.csv", 
                                output_dir, 
                                rxcui, 
                                sanitize(dea_name));
        let mut wtr = csv::Writer::from_path(output_file_path)?;
        //Write the header to the output file
        wtr.write_record(&header)?;
        //Write the concepts to the output file, calls function to extract the concept properties from the json
        write_concepts_to_csv(&mut wtr, &concepts_json)?;
        //Close the CSV writer
        wtr.flush()?;       
    }    

    Ok(())
}

fn write_concepts_to_csv(wtr: &mut csv::Writer<std::fs::File>, 
                        concepts_json: &RxNormConceptsResultJson) 
    -> Result<(), Box<dyn std::error::Error>> {
    
    for concept_group in &concepts_json.relatedGroup.conceptGroup {
        if let Some(concept_properties) = &concept_group.conceptProperties {
            for concept_property in concept_properties {
                let record = vec![
                    concept_property.rxcui.clone().unwrap_or_default(),
                    concept_property.name.clone().unwrap_or_default(),
                    concept_property.synonym.clone().unwrap_or_default(),
                    concept_property.tty.clone().unwrap_or_default(),
                    concept_property.language.clone().unwrap_or_default(),
                    concept_property.suppress.clone().unwrap_or_default(),
                    concept_property.umlscui.clone().unwrap_or_default(),
                    concept_property.psn.clone().unwrap_or_default(),
                ];
                wtr.write_record(&record)?;
            }
        }
    }

    Ok(())
}