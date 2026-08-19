use regex::Captures;
use rust_decimal::Decimal;

use super::const_regex::{
    BRAND_PACK_REGEX,
    COMPONENT_REGEX,
    OUTER_PACK_REGEX,
    PACK_ITEM_REGEX,
    PACK_ITEM_WITH_CONTAINER_REGEX,
    PRESENTATION_DATA_REGEX,
};

use super::dose_forms::{
    extract_dose_form,
    Container,
    DenominatorUnit,
    DrugComponent,
    MassUnit,
    PackItem,
    ParsedConcept,
    ParsedDrug,
    ParsedPack,
    Presentation,
    Strength,
    TimeUnit,
    VolumeUnit,
};

use super::parser_error::ParseError;


pub fn parse(
    rxcui: &str,
    tty: &str,
    data: &str,
) -> Result<ParsedConcept, ParseError> {
    match tty {
        "SCD" | "SBD" => {
            parse_drug(rxcui, tty, data).map(ParsedConcept::Drug)
        }

        "GPCK" | "BPCK" => {
            parse_pack(rxcui, tty, data).map(ParsedConcept::Pack)
        }

        _ => Err(ParseError::UnsupportedTty {
            tty: tty.to_string(),
            source: data.to_string(),
        }),
    }
}

pub fn parse_drug(
    rxcui: &str,
    tty: &str,
    source_name: &str,
) -> Result<ParsedDrug, ParseError> {
    let mut remaining = source_name.trim().to_string();

    let brand = parse_brand(&mut remaining, source_name)?;

    let presentation = parse_presentation(&mut remaining, source_name)?;

    
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

fn parse_component(
    text: &str,
    source_name: &str,
) -> Result<DrugComponent, ParseError> {
    let captures =
        COMPONENT_REGEX
            .captures(text)
            .ok_or_else(|| ParseError::InvalidComponent {
                component: text.to_string(),
                source: source_name.to_string(),
            })?;

    let ingredient_expression =
        capture_str(&captures, "ingredient", source_name)?
            .trim()
            .to_string();

    let value = capture_decimal(&captures, "value", source_name)?;

    let numerator_unit =
        match capture_str(&captures, "numerator", source_name)? {
            "MCG" => MassUnit::Microgram,
            "MG" => MassUnit::Milligram,
            "G" => MassUnit::Gram,

            unit => {
                return Err(ParseError::UnsupportedMassUnit {
                    unit: unit.to_string(),
                    source: source_name.to_string(),
                });
            }
        };

    let denominator_value = captures
        .name("denominator_value")
        .map(|capture| {
            let text = capture.as_str();

            text.parse::<Decimal>()
                .map_err(|_| ParseError::InvalidDecimal {
                    value: text.to_string(),
                    source: source_name.to_string(),
                })
        })
        .transpose()?;

    let denominator_unit = captures
        .name("denominator")
        .map(|capture| {
            match capture.as_str() {
                "ML" => Ok(DenominatorUnit::Milliliter),
                "HR" => Ok(DenominatorUnit::Hour),
                "ACTUAT" => Ok(DenominatorUnit::Actuation),

                unit => Err(ParseError::UnsupportedDenominatorUnit {
                    unit: unit.to_string(),
                    source: source_name.to_string(),
                }),
            }
        })
        .transpose()?;

    Ok(DrugComponent {
        ingredient_expression,
        strength: Strength {
            value,
            numerator_unit,
            denominator_value,
            denominator_unit,
        },
    })
}



fn parse_presentation(
    data: &mut String,
    source_name: &str,
) -> Result<Option<Presentation>, ParseError> {
    let Some(captures) =
        PRESENTATION_DATA_REGEX.captures(data.as_str())
    else {
        return Ok(None);
    };

    let value =
        capture_decimal(&captures, "presentation_value", source_name)?;

    let unit =
        capture_str(&captures, "presentation_unit", source_name)?;

    let presentation = match unit {
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
    };

    let drug = capture_str(&captures, "drug", source_name)?
        .trim()
        .to_string();

    *data = drug;

    Ok(Some(presentation))
}

fn parse_brand(
    data: &mut String,
    source_name: &str,
) -> Result<Option<String>, ParseError> {
    let captures = BRAND_PACK_REGEX
        .captures(data.as_str())
        .ok_or_else(|| ParseError::InvalidBrandSyntax {
            source: source_name.to_string(),
        })?;

    let brand = captures
        .name("brand")
        .map(|capture| capture.as_str().to_string());

    let generic = capture_str(
        &captures,
        "generic",
        source_name,
    )?
    .trim()
    .to_string();

    *data = generic;

    Ok(brand)
}

fn parse_pack(
    rxcui: &str,
    tty: &str,
    source_name: &str,
) -> Result<ParsedPack, ParseError> {
    let captures = OUTER_PACK_REGEX
        .captures(source_name)
        .ok_or_else(|| ParseError::InvalidPack {
            source: source_name.to_string(),
        })?;

    let items_text = capture_str(
        &captures,
        "items",
        source_name,
    )?;

    let brand = captures
        .name("pack_brand")
        .map(|capture| capture.as_str().to_string());

    let inner_tty = match tty {
        "GPCK" => "SCD",
        "BPCK" => "SBD",

        _ => {
            return Err(ParseError::UnsupportedTty {
                tty: tty.to_string(),
                source: source_name.to_string(),
            });
        }
    };

    let items = split_pack_items(items_text)
        .into_iter()
        .map(|item| {
            parse_pack_item(item, inner_tty, source_name)
        })
        .collect::<Result<Vec<_>, _>>()?;

    if items.is_empty() {
        return Err(ParseError::InvalidPack {
            source: source_name.to_string(),
        });
    }

    Ok(ParsedPack {
        rxcui: rxcui.to_string(),
        tty: tty.parse()?,
        source_name: source_name.to_string(),
        brand,
        items,
    })
}

fn parse_pack_item(
    item: &str,
    inner_tty: &str,
    source_name: &str,
) -> Result<PackItem, ParseError> {
    if let Some(captures) =
        PACK_ITEM_WITH_CONTAINER_REGEX.captures(item)
    {
        let count = capture_u32(
            &captures,
            "count",
            source_name,
        )?;

        let container_value = capture_decimal(
            &captures,
            "container_value",
            source_name,
        )?;

        let concept = capture_str(
            &captures,
            "concept",
            source_name,
        )?;

        let drug = parse_drug(
            "",
            inner_tty,
            concept,
        )?;

        return Ok(PackItem {
            count,
            container: Some(Container {
                value: container_value,
                unit: VolumeUnit::Milliliter,
            }),
            drug,
        });
    }

    if let Some(captures) = PACK_ITEM_REGEX.captures(item) {
        let count = capture_u32(
            &captures,
            "count",
            source_name,
        )?;

        let concept = capture_str(
            &captures,
            "concept",
            source_name,
        )?;

        let drug = parse_drug(
            "",
            inner_tty,
            concept,
        )?;

        return Ok(PackItem {
            count,
            container: None,
            drug,
        });
    }

    Err(ParseError::InvalidPackItem {
        item: item.to_string(),
        source: source_name.to_string(),
    })
}

fn split_pack_items(items: &str) -> Vec<&str> {
    let bytes = items.as_bytes();
    let mut results = Vec::new();

    let mut depth = 0_u32;
    let mut start = 0_usize;
    let mut index = 0_usize;

    while index < bytes.len() {
        match bytes[index] {
            b'(' => {
                depth += 1;
                index += 1;
            }

            b')' => {
                depth = depth.saturating_sub(1);
                index += 1;
            }

            _ if depth == 0
                && bytes[index..].starts_with(b" / ") =>
            {
                results.push(items[start..index].trim());
                index += 3;
                start = index;
            }

            _ => {
                index += 1;
            }
        }
    }

    let final_item = items[start..].trim();

    if !final_item.is_empty() {
        results.push(final_item);
    }

    results
}

fn capture_str<'source>(
    captures: &Captures<'source>,
    name: &'static str,
    source: &str,
) -> Result<&'source str, ParseError> {
    captures
        .name(name)
        .map(|capture| capture.as_str())
        .ok_or_else(|| ParseError::MissingCapture {
            capture: name,
            source: source.to_string(),
        })
}

fn capture_decimal(
    captures: &Captures<'_>,
    name: &'static str,
    source: &str,
) -> Result<Decimal, ParseError> {
    let value = captures
        .name(name)
        .ok_or_else(|| ParseError::MissingCapture {
            capture: name,
            source: source.to_string(),
        })?
        .as_str();

    value
        .parse::<Decimal>()
        .map_err(|_| ParseError::InvalidDecimal {
            value: value.to_string(),
            source: source.to_string(),
        })
}

fn decimal_to_u32(
    value: Decimal,
    source: &str,
) -> Result<u32, ParseError> {
    value
        .to_string()
        .parse::<u32>()
        .map_err(|_| ParseError::InvalidActuationCount {
            value,
            source: source.to_string(),
        })
}

fn capture_u32(
    captures: &Captures<'_>,
    name: &'static str,
    source: &str,
) -> Result<u32, ParseError> {
    let value = capture_str(captures, name, source)?;

    value
        .parse::<u32>()
        .map_err(|_| ParseError::InvalidDecimal {
            value: value.to_string(),
            source: source.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_generic_injection() {
        let parsed = parse(
            "1735003",
            "SCD",
            "2 ML fentanyl 0.05 MG/ML Injection",
        )
        .unwrap();

        let ParsedConcept::Drug(drug) = parsed else {
            panic!("expected drug");
        };

        assert_eq!(drug.rxcui, "1735003");
        assert!(drug.brand.is_none());
        assert_eq!(drug.components.len(), 1);
    }

    #[test]
    fn parses_branded_drug() {
        let parsed = parse(
            "example",
            "SBD",
            "fentanyl 0.1 MG Buccal Tablet [Fentora]",
        )
        .unwrap();

        let ParsedConcept::Drug(drug) = parsed else {
            panic!("expected drug");
        };

        assert_eq!(drug.brand.as_deref(), Some("Fentora"));
        assert_eq!(drug.components.len(), 1);
    }

    #[test]
    fn parses_combination_drug() {
        let parsed = parse(
            "example",
            "SCD",
            "acetaminophen 325 MG / codeine phosphate 30 MG Oral Tablet",
        )
        .unwrap();

        let ParsedConcept::Drug(drug) = parsed else {
            panic!("expected drug");
        };

        assert_eq!(drug.components.len(), 2);
    }

    #[test]
    fn splits_pack_items_only_at_depth_zero() {
        let text =
            "7 (perampanel 4 MG Oral Tablet) / 7 (perampanel 6 MG Oral Tablet)";

        let items = split_pack_items(text);

        assert_eq!(items.len(), 2);
    }

    #[test]
    fn parses_pack() {
        let parsed = parse(
            "example",
            "GPCK",
            "{7 (perampanel 4 MG Oral Tablet) / 7 (perampanel 6 MG Oral Tablet) } Pack",
        )
        .unwrap();

        let ParsedConcept::Pack(pack) = parsed else {
            panic!("expected pack");
        };

        assert_eq!(pack.items.len(), 2);
        assert_eq!(pack.items[0].count, 7);
        assert_eq!(pack.items[1].count, 7);
    }
}