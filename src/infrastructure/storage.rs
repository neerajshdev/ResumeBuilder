use crate::application::repository::ResumeRepository;
use crate::domain::Resume;
use std::error::Error;
use std::cell::RefCell;
use dioxus::prelude::*;

// In-memory repository implementation for Dioxus applications
pub struct InMemoryResumeRepository {
    resume_signal: RefCell<Signal<Option<Resume>>>,
}

impl InMemoryResumeRepository {
    pub fn new(resume_signal: Signal<Option<Resume>>) -> Self {
        Self {
            resume_signal: RefCell::new(resume_signal),
        }
    }
}

impl ResumeRepository for InMemoryResumeRepository {
    fn save(&self, resume: &Resume) -> Result<(), Box<dyn Error>> {
        self.resume_signal.borrow_mut().write().replace(resume.clone());
        Ok(())
    }
    
    fn load(&self) -> Result<Resume, Box<dyn Error>> {
        match self.resume_signal.borrow().read().clone() {
            Some(resume) => Ok(resume),
            None => Ok(Resume::default()),
        }
    }
    
    fn exists(&self) -> bool {
        self.resume_signal.borrow().read().is_some()
    }
}

// Local storage repository implementation for web applications
#[cfg(feature = "web")]
pub struct LocalStorageResumeRepository {
    storage_key: String,
}

#[cfg(feature = "web")]
impl LocalStorageResumeRepository {
    pub fn new(storage_key: &str) -> Self {
        Self {
            storage_key: storage_key.to_string(),
        }
    }
    
    fn get_local_storage(&self) -> Result<web_sys::Storage, Box<dyn Error>> {
        let window = web_sys::window().ok_or("Failed to get window")?;
        window
            .local_storage()
            .map_err(|_| "Failed to get local storage")?
            .ok_or_else(|| "Local storage not available".into())
    }
}

#[cfg(feature = "web")]
impl ResumeRepository for LocalStorageResumeRepository {
    fn save(&self, resume: &Resume) -> Result<(), Box<dyn Error>> {
        let storage = self.get_local_storage()?;
        let json = serde_json::to_string(resume)?;
        storage
            .set_item(&self.storage_key, &json)
            .map_err(|_| "Failed to save to local storage")?;
        Ok(())
    }
    
    fn load(&self) -> Result<Resume, Box<dyn Error>> {
        let storage = self.get_local_storage()?;
        let json = storage
            .get_item(&self.storage_key)
            .map_err(|_| "Failed to load from local storage")?
            .ok_or("Resume not found in storage")?;
        
        let resume: Resume = serde_json::from_str(&json)?;
        Ok(resume)
    }
    
    fn exists(&self) -> bool {
        if let Ok(storage) = self.get_local_storage() {
            if let Ok(Some(_)) = storage.get_item(&self.storage_key) {
                return true;
            }
        }
        false
    }
} 