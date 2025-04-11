use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Core domain entities for the Resume Builder application

// Resume data model - root aggregate
#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Resume {
    pub personal_info: PersonalInfo,
    pub education: Vec<Education>,
    pub experience: Vec<Experience>,
    pub skills: Skills,
    pub projects: Vec<Project>,
    pub theme: ResumeTheme,
}

// Personal information section
#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct PersonalInfo {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub website: String,
    pub linkedin: String,
    pub github: String,
    pub location: String,
    pub summary: String,
}

// Education section
#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Education {
    pub institution: String,
    pub degree: String,
    pub field_of_study: String,
    pub start_date: String,
    pub end_date: String,
    pub location: String,
    pub description: String,
    pub gpa: String,
}

// Work experience section
#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Experience {
    pub company: String,
    pub position: String,
    pub start_date: String,
    pub end_date: String,
    pub location: String,
    pub description: String,
    pub achievements: Vec<String>,
    pub is_current: bool,
}

// Skills section
#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Skills {
    pub categories: HashMap<String, Vec<String>>,
}

// Project section
#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub start_date: String,
    pub end_date: String,
    pub technologies: Vec<String>,
    pub url: String,
    pub achievements: Vec<String>,
}

// Resume theme
#[derive(Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResumeTheme {
    #[default]
    Professional,
    Minimal,
    Creative,
    Modern,
}

impl ResumeTheme {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Professional,
            Self::Minimal,
            Self::Creative,
            Self::Modern,
        ]
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Self::Professional => "Professional",
            Self::Minimal => "Minimal",
            Self::Creative => "Creative",
            Self::Modern => "Modern",
        }
    }
} 