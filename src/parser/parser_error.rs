use rust_decimal::Decimal;

#[derive(Debug)]
pub enum ParseError {
    UnsupportedTty {
        tty: String,
        source: String,
    },

    InvalidBrandSyntax {
        source: String,
    },

    MissingCapture {
        capture: &'static str,
        source: String,
    },

    UnknownDoseForm {
        remaining: String,
        source: String,
    },

    NoComponents {
        source: String,
    },

    InvalidComponent {
        component: String,
        source: String,
    },

    InvalidDecimal {
        value: String,
        source: String,
    },

    UnsupportedMassUnit {
        unit: String,
        source: String,
    },

    UnsupportedDenominatorUnit {
        unit: String,
        source: String,
    },

    UnsupportedPresentationUnit {
        unit: String,
        source: String,
    },

    InvalidActuationCount {
        value: Decimal,
        source: String,
    },

    InvalidPack {
        source: String,
    },

    InvalidPackItem {
        item: String,
        source: String,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::UnsupportedTty { tty, source } => {
                write!(
                    formatter,
                    "unsupported TTY `{tty}` while parsing `{source}`"
                )
            }

            Self::InvalidBrandSyntax { source } => {
                write!(
                    formatter,
                    "invalid brand syntax in `{source}`"
                )
            }

            Self::MissingCapture { capture, source } => {
                write!(
                    formatter,
                    "missing regex capture `{capture}` in `{source}`"
                )
            }

            Self::UnknownDoseForm {
                remaining,
                source,
            } => {
                write!(
                    formatter,
                    "unknown dose form in `{source}`; remaining text: `{remaining}`"
                )
            }

            Self::NoComponents { source } => {
                write!(
                    formatter,
                    "no drug components found in `{source}`"
                )
            }

            Self::InvalidComponent {
                component,
                source,
            } => {
                write!(
                    formatter,
                    "invalid component `{component}` in `{source}`"
                )
            }

            Self::InvalidDecimal { value, source } => {
                write!(
                    formatter,
                    "invalid decimal `{value}` in `{source}`"
                )
            }

            Self::UnsupportedMassUnit { unit, source } => {
                write!(
                    formatter,
                    "unsupported mass unit `{unit}` in `{source}`"
                )
            }

            Self::UnsupportedDenominatorUnit {
                unit,
                source,
            } => {
                write!(
                    formatter,
                    "unsupported denominator unit `{unit}` in `{source}`"
                )
            }

            Self::UnsupportedPresentationUnit {
                unit,
                source,
            } => {
                write!(
                    formatter,
                    "unsupported presentation unit `{unit}` in `{source}`"
                )
            }

            Self::InvalidActuationCount {
                value,
                source,
            } => {
                write!(
                    formatter,
                    "invalid actuation count `{value}` in `{source}`"
                )
            }

            Self::InvalidPack { source } => {
                write!(
                    formatter,
                    "invalid pack syntax in `{source}`"
                )
            }

            Self::InvalidPackItem { item, source } => {
                write!(
                    formatter,
                    "invalid pack item `{item}` in `{source}`"
                )
            }
        }
    }
}

impl std::error::Error for ParseError {}