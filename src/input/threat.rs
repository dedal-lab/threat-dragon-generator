use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    Open,
    NotApplicable,
    Mitigated,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Status::Open => write!(f, "Open"),
            Status::NotApplicable => write!(f, "NotApplicable"),
            Status::Mitigated => write!(f, "Mitigated"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Severity {
    Low,
    Medium,
    High,
}

impl Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Severity::Low => write!(f, "Low"),
            Severity::Medium => write!(f, "Medium"),
            Severity::High => write!(f, "High"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TypeThreat {
    Spoofing,
    Tampering,
    Repudiation,
    InformationDisclosure,
    DenialOfService,
    ElevationOfPrivilege,
}

impl Display for TypeThreat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TypeThreat::Spoofing => write!(f, "Spoofing"),
            TypeThreat::Tampering => write!(f, "Tampering"),
            TypeThreat::Repudiation => write!(f, "Repudiation"),
            TypeThreat::InformationDisclosure => write!(f, "Information disclosure"),
            TypeThreat::DenialOfService => write!(f, "Denial of service"),
            TypeThreat::ElevationOfPrivilege => write!(f, "Elevation of privilege"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Threat {
    pub title: String,
    pub status: Status,
    pub severity: Severity,
    #[serde(rename = "type")]
    pub type_field: TypeThreat,
    pub description: String,
    pub mitigation: String,
}
