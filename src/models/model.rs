use serde::Serialize;

use crate::parser::ParsedDrug;

pub trait Model {
    type Shape: Serialize;

    /// Determine whether this model recognizes the RXCUI.
    fn rxcui_is(&self, drug: &ParsedDrug) -> bool;

    /// Determine whether the parsed concept matches this model.
    fn concept_is(&self, drug: &ParsedDrug) -> bool;

    /// Determine whether the dose form belongs in this model.
    fn dose_form_is(&self, drug: &ParsedDrug) -> bool;

    /// Apply every inclusion rule.
    fn accept<'a>(
        &self,
        drug: &'a ParsedDrug,
    ) -> Option<&'a ParsedDrug> {
        if self.rxcui_is(drug)
            && self.concept_is(drug)
            && self.dose_form_is(drug)
        {
            Some(drug)
        } else {
            None
        }
    }

    /// Convert an accepted ParsedDrug into the model's CSV structure.
    fn shape(&self, drug: &ParsedDrug) -> Self::Shape;
}