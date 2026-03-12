use std::fmt;

/// Filing status for tax calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilingStatus {
    Single,
    MarriedFilingJointly,
    MarriedFilingSeparately,
    HeadOfHousehold,
    QualifyingSurvivingSpouse,
}

impl FilingStatus {
    pub fn parse(s: &str) -> Result<Self, DataError> {
        match s.to_lowercase().replace('-', "_").as_str() {
            "single" | "s" => Ok(Self::Single),
            "married_filing_jointly" | "mfj" => Ok(Self::MarriedFilingJointly),
            "married_filing_separately" | "mfs" => Ok(Self::MarriedFilingSeparately),
            "head_of_household" | "hoh" => Ok(Self::HeadOfHousehold),
            "qualifying_surviving_spouse" | "qss" | "qualifying_widow" | "qualifying_widower" => {
                Ok(Self::QualifyingSurvivingSpouse)
            }
            _ => Err(DataError::InvalidParams(format!(
                "Unknown filing status: '{s}'"
            ))),
        }
    }

    /// All variants in canonical order.
    pub fn all() -> &'static [FilingStatus] {
        &[
            Self::Single,
            Self::MarriedFilingJointly,
            Self::MarriedFilingSeparately,
            Self::HeadOfHousehold,
            Self::QualifyingSurvivingSpouse,
        ]
    }
}

impl fmt::Display for FilingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Single => write!(f, "single"),
            Self::MarriedFilingJointly => write!(f, "married_filing_jointly"),
            Self::MarriedFilingSeparately => write!(f, "married_filing_separately"),
            Self::HeadOfHousehold => write!(f, "head_of_household"),
            Self::QualifyingSurvivingSpouse => write!(f, "qualifying_surviving_spouse"),
        }
    }
}

/// Errors from the data module.
#[derive(Debug, Clone)]
pub enum DataError {
    UnknownCategory(String),
    UnknownKey(String),
    UnsupportedYear(u32),
    InvalidParams(String),
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownCategory(c) => write!(f, "Unknown category: '{c}'"),
            Self::UnknownKey(k) => write!(f, "Unknown key: '{k}'"),
            Self::UnsupportedYear(y) => write!(f, "Unsupported year: {y}"),
            Self::InvalidParams(msg) => write!(f, "Invalid parameters: {msg}"),
        }
    }
}

impl std::error::Error for DataError {}

/// Parameters for string-based lookup dispatch.
pub struct LookupParams {
    pub filing_status: Option<String>,
}

/// Filter for coverage queries.
pub struct CoverageFilter {
    pub category: Option<String>,
    pub tag: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filing_status_parse_variants() {
        assert_eq!(FilingStatus::parse("single").unwrap(), FilingStatus::Single);
        assert_eq!(
            FilingStatus::parse("mfj").unwrap(),
            FilingStatus::MarriedFilingJointly
        );
        assert_eq!(
            FilingStatus::parse("MFS").unwrap(),
            FilingStatus::MarriedFilingSeparately
        );
        assert_eq!(
            FilingStatus::parse("hoh").unwrap(),
            FilingStatus::HeadOfHousehold
        );
        assert_eq!(
            FilingStatus::parse("qss").unwrap(),
            FilingStatus::QualifyingSurvivingSpouse
        );
        assert_eq!(
            FilingStatus::parse("married-filing-jointly").unwrap(),
            FilingStatus::MarriedFilingJointly
        );
    }

    #[test]
    fn filing_status_invalid() {
        assert!(FilingStatus::parse("invalid").is_err());
    }

    #[test]
    fn filing_status_display() {
        assert_eq!(FilingStatus::Single.to_string(), "single");
        assert_eq!(
            FilingStatus::MarriedFilingJointly.to_string(),
            "married_filing_jointly"
        );
    }
}
