use rust_decimal::Decimal;

use super::parser_error::ParseError;


#[derive(Debug)]
pub enum ParsedConcept {
    Drug(ParsedDrug),
    Pack(ParsedPack),
}

#[derive(Debug)]
pub struct ParsedDrug {
    pub rxcui: String,
    pub tty: TermType,
    pub source_name: String,
    pub brand: Option<String>,
    pub presentation: Option<Presentation>,
    pub dose_form: DoseForm,
    pub components: Vec<DrugComponent>,
}

#[derive(Debug)]
pub struct ParsedPack {
    pub rxcui: String,
    pub tty: TermType,
    pub source_name: String,
    pub brand: Option<String>,
    pub items: Vec<PackItem>,
}



#[derive(Debug)]
pub struct PackItem {
    pub count: u32,
    pub container: Option<Container>,
    pub drug: ParsedDrug,
}

#[derive(Debug)]
pub struct Container {
    pub value: Decimal,
    pub unit: VolumeUnit,
}

#[derive(Debug)]
pub struct DrugComponent {
    pub ingredient_expression: String,
    pub strength: Strength,
}

#[derive(Debug)]
pub struct Strength {
    pub value: Decimal,
    pub numerator_unit: MassUnit,
    pub denominator_value: Option<Decimal>,
    pub denominator_unit: Option<DenominatorUnit>,
}

#[derive(Debug)]
pub enum Presentation {
    Volume {
        value: Decimal,
        unit: VolumeUnit,
    },
    Duration {
        value: Decimal,
        unit: TimeUnit,
    },
    ActuationCount {
        count: u32,
    },
}

#[derive(Debug)]
pub enum MassUnit {
    Microgram,
    Milligram,
    Gram,
}

#[derive(Debug)]
pub enum DenominatorUnit {
    Milliliter,
    Hour,
    Actuation,
}

#[derive(Debug)]
pub enum VolumeUnit {
    Milliliter,
}

#[derive(Debug)]
pub enum TimeUnit {
    Hour,
}

#[derive(Debug)]
pub enum TermType {
    Scd,
    Sbd,
    Gpck,
    Bpck,
}

impl std::str::FromStr for TermType {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "SCD" => Ok(Self::Scd),
            "SBD" => Ok(Self::Sbd),
            "GPCK" => Ok(Self::Gpck),
            "BPCK" => Ok(Self::Bpck),

            _ => Err(ParseError::UnsupportedTty {
                tty: value.to_string(),
                source: String::new(),
            }),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoseForm {
    pub route: Route,
    pub physical_form: PhysicalForm,
    pub release: ReleaseType,
    pub delivery_device: Option<DeliveryDevice>,
    pub rxnorm_text: String,
}

pub fn extract_dose_form(
    name: &str,
) -> Option<(&str, DoseForm)> {
    DOSE_FORMS.iter().find_map(|dose_form_text| {
        let dose_form_text = *dose_form_text;

        let component_text = name
            .strip_suffix(dose_form_text)?
            .trim_end();

        Some((
            component_text,
            build_dose_form(dose_form_text),
        ))
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    Oral,
    Buccal,
    Sublingual,
    Nasal,
    Rectal,
    Topical,
    Transdermal,
    Parenteral,
    Mucosal,
    Unspecified,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalForm {
    Tablet,
    Capsule,
    Solution,
    Suspension,
    Film,
    Spray,
    Lozenge,
    Suppository,
    Gel,
    Enema,
    System,
    Injection,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseType {
    Immediate,
    Extended,
    Disintegrating,
    Effervescent,
    Unspecified,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeliveryDevice {
    PrefilledSyringe,
    AutoInjector,
    Cartridge,
    MeteredDose,
}



pub const DOSE_FORMS: &[&str] = &[
    "Extended Release Oral Tablet",
    "Extended Release Oral Capsule",
    "Disintegrating Oral Tablet",
    "Tablet for Oral Suspension",
    "Effervescent Oral Tablet",
    "Extended Release Suspension",
    "Metered Dose Nasal Spray",
    "Injectable Suspension",
    "Injectable Solution",
    "Transdermal System",
    "Rectal Suppository",
    "Prefilled Syringe",
    "Sublingual Tablet",
    "Sublingual Film",
    "Buccal Tablet",
    "Buccal Film",
    "Mucosal Spray",
    "Nasal Spray",
    "Topical Solution",
    "Oral Suspension",
    "Oral Solution",
    "Oral Capsule",
    "Oral Lozenge",
    "Oral Tablet",
    "Rectal Gel",
    "Auto-Injector",
    "Cartridge",
    "Injection",
    "Enema",
];

fn build_dose_form(text: &str) -> DoseForm {
    let route = if text.contains("Oral") {
        Route::Oral
    } else if text.contains("Buccal") {
        Route::Buccal
    } else if text.contains("Sublingual") {
        Route::Sublingual
    } else if text.contains("Nasal") {
        Route::Nasal
    } else if text.contains("Rectal") || text == "Enema" {
        Route::Rectal
    } else if text.contains("Topical") {
        Route::Topical
    } else if text.contains("Transdermal") {
        Route::Transdermal
    } else if text.contains("Mucosal") {
        Route::Mucosal
    } else if matches!(
        text,
        "Injection"
            | "Injectable Solution"
            | "Injectable Suspension"
            | "Prefilled Syringe"
            | "Auto-Injector"
            | "Cartridge"
    ) {
        Route::Parenteral
    } else {
        Route::Unspecified
    };

    let physical_form = if text.contains("Tablet") {
        PhysicalForm::Tablet
    } else if text.contains("Capsule") {
        PhysicalForm::Capsule
    } else if text.contains("Solution") {
        PhysicalForm::Solution
    } else if text.contains("Suspension") {
        PhysicalForm::Suspension
    } else if text.contains("Film") {
        PhysicalForm::Film
    } else if text.contains("Spray") {
        PhysicalForm::Spray
    } else if text.contains("Lozenge") {
        PhysicalForm::Lozenge
    } else if text.contains("Suppository") {
        PhysicalForm::Suppository
    } else if text.contains("Gel") {
        PhysicalForm::Gel
    } else if text == "Enema" {
        PhysicalForm::Enema
    } else if text.contains("System") {
        PhysicalForm::System
    } else if text.contains("Injection") {
        PhysicalForm::Injection
    } else {
        PhysicalForm::Other
    };

    let release = if text.contains("Extended Release") {
        ReleaseType::Extended
    } else if text.contains("Disintegrating") {
        ReleaseType::Disintegrating
    } else if text.contains("Effervescent") {
        ReleaseType::Effervescent
    } else {
        ReleaseType::Immediate
    };

    let delivery_device = if text == "Prefilled Syringe" {
        Some(DeliveryDevice::PrefilledSyringe)
    } else if text == "Auto-Injector" {
        Some(DeliveryDevice::AutoInjector)
    } else if text == "Cartridge" {
        Some(DeliveryDevice::Cartridge)
    } else if text == "Metered Dose Nasal Spray" {
        Some(DeliveryDevice::MeteredDose)
    } else {
        None
    };

    DoseForm {
        route,
        physical_form,
        release,
        delivery_device,
        rxnorm_text: text.to_string(),
    }
}