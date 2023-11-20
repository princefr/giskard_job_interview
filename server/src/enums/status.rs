use std::fmt::Display;

use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MeleniumStatus {
    Refueling,
    Waiting
}


impl Display for MeleniumStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MeleniumStatus::Refueling => write!(f, "Refueling"),
            MeleniumStatus::Waiting => write!(f, "Waiting"),
        }
    }
}