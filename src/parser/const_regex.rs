
use std::sync::LazyLock as Lazy;
use regex::Regex;


pub static OUTER_RULE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^\{\s*(?<items>.+?)\s*\}\s+Pack(?:\s+\[(?<pack_brand>[^\[\]]+)\])?$"
    ).unwrap()
});

pub static ITEM_SINGLE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^\{\s*(?<items>.+?)\s*\}\s+Pack(?:\s+\[(?<pack_brand>[^\[\]]+)\])?$"
    ).unwrap()
});

pub static ITEM_MULTIPLE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^(?<count>\d+)\s+\((?<container_value>\d+(?:\.\d+)?)\s+(?<container_unit>ML)\)\s+\((?<concept>.+)\)$"
    ).unwrap()
});

pub static BRAND_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^(?<generic>.+?)(?:\s+\[(?<brand>[^\[\]]+)\])?$"
    ).unwrap()
});

pub static PRESENTATION_DATA_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^(?<presentation_value>\d+(?:\.\d+)?)\s+(?<presentation_unit>ML|HR|ACTUAT)\s+(?<drug>.+)$"
    ).unwrap()
});