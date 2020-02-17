use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Defines the level of severity for the event.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum SeverityLevel {
    Verbose,
    Information,
    Warning,
    Error,
    Critical,
}

#[cfg(test)]
mod tests {
    use serde_json::to_string;

    use super::*;

    #[test]
    fn it_json_serializes_valid_constants() {
        // The JSON-serialized values must match the value of `constantName` in
        // `schema/SeverityLevel.json`.
        //
        // Regression test for appinsights-rs#18.
        assert_eq!(to_string(&SeverityLevel::Verbose).unwrap(), r#""Verbose""#);
        assert_eq!(to_string(&SeverityLevel::Information).unwrap(), r#""Information""#);
        assert_eq!(to_string(&SeverityLevel::Warning).unwrap(), r#""Warning""#);
        assert_eq!(to_string(&SeverityLevel::Error).unwrap(), r#""Error""#);
        assert_eq!(to_string(&SeverityLevel::Critical).unwrap(), r#""Critical""#);
    }
}
