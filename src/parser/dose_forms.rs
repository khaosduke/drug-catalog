

struct Route {
    route_text: String,
}

struct DoseForm {
    route: Route,
    physical_form: PhysicalForm,
    release: ReleaseType,
    delivery_device: Option<DeliveryDevice>,
    rxnorm_text: String,
}



const DOSE_FORMS: &[&str] = &[
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