use crate::models::model::Model;


pub struct AcceptedDrug {
    pub rxcui: String,
    pub tty: String,
    pub source_name: String,
    pub brand: Option<String>,
    pub presentation: Option<String>,
    pub dose_form: String,
    pub components: Vec<AcceptedDrugComponent>,
}

impl Model for AcceptedDrug {

    fn dose_form_is(&self, drug: &ParsedDrug) -> bool {
        !matches!(self.dose_form.route, 
            DoseForm::Route::Buccal
        )
        ||
        !matches!(self.dose_form.physical_form, 
            DoseForm::PhysicalForm::Lozenge
            | DoseForm::PhysicalForm::Enema 
            | DoseForm::PhysicalForm::Film
        )
        ||
        !matches!(self.dose_form.release, 
            DoseForm::ReleaseType::ExtendedRelease
            | DoseForm::ReleaseType::Effervescent
        )

    }

    
}