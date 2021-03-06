//! Scalability mode.

use once_cell::sync::OnceCell;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

/// Scalability mode.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub struct ScalabilityMode {
    /// Number of spatial layers.
    pub spatial_layers: u8,
    /// Number of temporal layers.
    pub temporal_layers: u8,
    /// K-SVC mode.
    pub ksvc: bool,
}

impl Default for ScalabilityMode {
    fn default() -> Self {
        Self {
            spatial_layers: 1,
            temporal_layers: 1,
            ksvc: false,
        }
    }
}

/// Error that caused [`ScalabilityMode`] parsing error.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum ParseScalabilityModeError {
    /// Invalid input string
    #[error("Invalid input string")]
    InvalidInput,
}

impl FromStr for ScalabilityMode {
    type Err = ParseScalabilityModeError;

    fn from_str(scalability_mode: &str) -> Result<Self, Self::Err> {
        static SCALABILITY_MODE_REGEX: OnceCell<Regex> = OnceCell::new();

        SCALABILITY_MODE_REGEX
            .get_or_init(|| Regex::new(r"^[LS]([1-9][0-9]?)T([1-9][0-9]?)(_KEY)?").unwrap())
            .captures(scalability_mode)
            .map(|captures| ScalabilityMode {
                spatial_layers: captures.get(1).unwrap().as_str().parse().unwrap(),
                temporal_layers: captures.get(2).unwrap().as_str().parse().unwrap(),
                ksvc: captures.get(3).is_some(),
            })
            .ok_or(ParseScalabilityModeError::InvalidInput)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_scalability_modes() {
        assert_eq!(
            "L1T3".parse(),
            Ok(ScalabilityMode {
                spatial_layers: 1,
                temporal_layers: 3,
                ksvc: false
            }),
        );

        assert_eq!(
            "L3T2_KEY".parse(),
            Ok(ScalabilityMode {
                spatial_layers: 3,
                temporal_layers: 2,
                ksvc: true
            }),
        );

        assert_eq!(
            "S2T3".parse(),
            Ok(ScalabilityMode {
                spatial_layers: 2,
                temporal_layers: 3,
                ksvc: false
            }),
        );

        assert_eq!(
            "foo".parse::<ScalabilityMode>(),
            Err(ParseScalabilityModeError::InvalidInput),
        );

        assert_eq!(
            "ull".parse::<ScalabilityMode>(),
            Err(ParseScalabilityModeError::InvalidInput),
        );

        assert_eq!(
            "S0T3".parse::<ScalabilityMode>(),
            Err(ParseScalabilityModeError::InvalidInput),
        );

        assert_eq!(
            "S1T0".parse::<ScalabilityMode>(),
            Err(ParseScalabilityModeError::InvalidInput),
        );

        assert_eq!(
            "L20T3".parse(),
            Ok(ScalabilityMode {
                spatial_layers: 20,
                temporal_layers: 3,
                ksvc: false
            }),
        );

        assert_eq!(
            "S200T3".parse::<ScalabilityMode>(),
            Err(ParseScalabilityModeError::InvalidInput),
        );

        assert_eq!(
            "L4T7_KEY_SHIFT".parse(),
            Ok(ScalabilityMode {
                spatial_layers: 4,
                temporal_layers: 7,
                ksvc: true
            }),
        );
    }
}
