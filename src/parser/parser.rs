mod const_regex;
use const_regex::{BRAND_PACK_REGEX, 
                ITEM_MULTIPLE_REGEX, 
                ITEM_SINGLE_REGEX, 
                OUTER_RULE_REGEX, 
                PRESENTATION_DATA_REGEX};


pub fn parse(&mut self,&tty: &str, data: &str) -> Result<AST, ParseError> {
    // Implementation of the parse function
    // This function will parse the input and return an Abstract Syntax Tree (AST)
    // or a ParseError if the parsing fails.

    //Dispatch by TTY
    match tty {
        "SCD" | "SBD" => parse_drug(data),
        "GPCK" | "BPCK" => parse_pack(data),
        _ => unsupported(data),
    }
}
pub fn parse_drug(
    rxcui: &str,
    tty: &str,
    source_name: &str,
) -> Result<ParsedDrug, ParseError> {
    let mut remaining = source_name.trim().to_string();

    let brand = parse_brand(&mut remaining)?;

    let presentation = parse_presentation(&mut remaining)?;

    
    // 3. Remove the dose-form suffix.
    let (component_text, dose_form) =
        extract_dose_form(&remaining).ok_or_else(|| ParseError::UnknownDoseForm {
            remaining: remaining.clone(),
            source: source_name.to_string(),
        })?;

    // Own the text so we're no longer borrowing from `remaining`.
    let component_text = component_text.trim().to_string();

    // 4. Parse every ingredient/strength component.
    let components = component_text
        .split(" / ")
        .map(|component| parse_component(component, source_name))
        .collect::<Result<Vec<_>, _>>()?;

    if components.is_empty() {
        return Err(ParseError::NoComponents {
            source: source_name.to_string(),
        });
    }

    Ok(ParsedDrug {
        rxcui: rxcui.to_string(),
        tty: tty.parse()?,
        source_name: source_name.to_string(),
        brand,
        presentation,
        dose_form,
        components,
    })
}
fn parse_presentation(data: &mut str) -> Result<Presentation, ParseError> {
    // 2. Extract optional leading presentation.
    let presentation = if let Some(captures) = PRESENTATION.captures(*data) {
        let value = capture_decimal(&captures, "value", source_name)?;
        let unit = capture_str(&captures, "unit", source_name)?;

        remaining = capture_str(&captures, "drug", source_name)?.to_string();

        Some(match unit {
            "ML" => Presentation::Volume {
                value,
                unit: VolumeUnit::Milliliter,
            },

            "HR" => Presentation::Duration {
                value,
                unit: TimeUnit::Hour,
            },

            "ACTUAT" => Presentation::ActuationCount {
                count: decimal_to_u32(value, source_name)?,
            },

            _ => {
                return Err(ParseError::UnsupportedPresentationUnit {
                    unit: unit.to_string(),
                    source: source_name.to_string(),
                });
            }
        })
    } else {
        None
    };

}
fn parse_brand(data: &mut str) -> Result<String, ParseError> { 
    // 1. Extract optional trailing brand.
    let brand_captures = BRAND_PACK_REGEX
        .captures(*data)
        .ok_or_else(|| ParseError::InvalidBrandSyntax {
            source: source_name.to_string(),
        })?;

    let brand = brand_captures
        .name("brand")
        .map(|capture| capture.as_str().to_string());

    //Spit out the generic part of the brand and trim it
    //Remember we are mutating the data so we can parse the rest of the string    
    *data = brand_captures
        .name("generic")
        .ok_or_else(|| ParseError::MissingCapture {
            capture: "generic",
            source: source_name.to_string(),
        })?
        .as_str()
        .trim()
        .to_string();    

    Ok(brand.to_string())
}
fn parse_pack(data: &str) -> Result<AST, ParseError> {
    // Implementation of the pack parsing logic
    // This function will parse pack-related data and return an AST or a ParseError.
    Ok(AST::Pack(data.to_string())) // Placeholder implementation
}
fn unsupported(text: &str) -> Result<AST, ParseError> {
    // Handle unsupported TTY cases
    Err(ParseError::UnsupportedTTY)
}