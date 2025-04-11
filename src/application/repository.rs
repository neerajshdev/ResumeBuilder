use crate::domain::Resume;
use std::error::Error;

// Repository trait defines operations for resume storage
pub trait ResumeRepository {
    // Save a resume to storage
    fn save(&self, resume: &Resume) -> Result<(), Box<dyn Error>>;
    
    // Load a resume from storage
    fn load(&self) -> Result<Resume, Box<dyn Error>>;
    
    // Check if a resume exists in storage
    fn exists(&self) -> bool;
} 