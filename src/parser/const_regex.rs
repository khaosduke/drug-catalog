use regex::Regex;
use std::sync::LazyLock as Lazy;

pub static OUTER_PACK_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^\{\s*(?<items>.+?)\s*\}\s+Pack(?:\s+\[(?<pack_brand>[^\[\]]+)\])?$"
    )
    .unwrap()
});

pub static PACK_ITEM_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^(?<count>\d+)\s+\((?<concept>.+)\)$"
    )
    .unwrap()
});

pub static PACK_ITEM_WITH_CONTAINER_REGEX: Lazy<Regex> =
    Lazy::new(|| {
        Regex::new(
            r"^(?<count>\d+)\s+\((?<container_value>\d+(?:\.\d+)?)\s+(?<container_unit>ML)\)\s+\((?<concept>.+)\)$"
        )
        .unwrap()
    });

pub static BRAND_PACK_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^(?<generic>.+?)(?:\s+\[(?<brand>[^\[\]]+)\])?$"
    )
    .unwrap()
});

pub static PRESENTATION_DATA_REGEX: Lazy<Regex> =
    Lazy::new(|| {
        Regex::new(
            r"^(?<presentation_value>\d+(?:\.\d+)?)\s+(?<presentation_unit>ML|HR|ACTUAT)\s+(?<drug>.+)$"
        )
        .unwrap()
    });

pub static COMPONENT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^(?<ingredient>.+?)\s+(?<value>\d+(?:\.\d+)?)\s*(?<numerator>MCG|MG|G)(?:/(?:(?<denominator_value>\d+(?:\.\d+)?)\s*)?(?<denominator>ML|HR|ACTUAT))?$"
    )
    .unwrap()
});