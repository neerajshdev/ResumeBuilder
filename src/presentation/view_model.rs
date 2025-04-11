use crate::application::ResumeUseCase;
use crate::domain::{Resume, PersonalInfo, Education, Experience, Project, Skills, ResumeTheme};
use dioxus::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

// View model for the Resume Builder application
pub struct ResumeViewModel {
    resume_signal: RefCell<Signal<Resume>>,
    use_case: Rc<ResumeUseCase>,
}

impl ResumeViewModel {
    pub fn new(use_case: Rc<ResumeUseCase>) -> Self {
        let resume = use_case.create_new_resume();
        Self {
            resume_signal: RefCell::new(Signal::new(resume)),
            use_case,
        }
    }
    
    pub fn resume(&self) -> Resume {
        self.resume_signal.borrow().read().clone()
    }
    
    pub fn load(&self) {
        if let Ok(resume) = self.use_case.load_resume() {
            self.resume_signal.borrow_mut().set(resume);
        }
    }
    
    pub fn save(&self) {
        let resume = self.resume_signal.borrow().read().clone();
        let _ = self.use_case.save_resume(&resume);
    }
    
    pub fn update_personal_info(&self, info: PersonalInfo) {
        let mut resume = self.resume_signal.borrow().read().clone();
        self.use_case.update_personal_info(&mut resume, info);
        self.resume_signal.borrow_mut().set(resume);
    }
    
    pub fn add_education(&self, education: Education) {
        let mut resume = self.resume_signal.borrow().read().clone();
        self.use_case.add_education(&mut resume, education);
        self.resume_signal.borrow_mut().set(resume);
    }
    
    pub fn update_education(&self, index: usize, education: Education) {
        let mut resume = self.resume_signal.borrow().read().clone();
        if self.use_case.update_education(&mut resume, index, education).is_ok() {
            self.resume_signal.borrow_mut().set(resume);
        }
    }
    
    pub fn remove_education(&self, index: usize) {
        let mut resume = self.resume_signal.borrow().read().clone();
        if self.use_case.remove_education(&mut resume, index).is_ok() {
            self.resume_signal.borrow_mut().set(resume);
        }
    }
    
    pub fn add_experience(&self, experience: Experience) {
        let mut resume = self.resume_signal.borrow().read().clone();
        self.use_case.add_experience(&mut resume, experience);
        self.resume_signal.borrow_mut().set(resume);
    }
    
    pub fn update_experience(&self, index: usize, experience: Experience) {
        let mut resume = self.resume_signal.borrow().read().clone();
        if self.use_case.update_experience(&mut resume, index, experience).is_ok() {
            self.resume_signal.borrow_mut().set(resume);
        }
    }
    
    pub fn remove_experience(&self, index: usize) {
        let mut resume = self.resume_signal.borrow().read().clone();
        if self.use_case.remove_experience(&mut resume, index).is_ok() {
            self.resume_signal.borrow_mut().set(resume);
        }
    }
    
    pub fn update_skills(&self, skills: Skills) {
        let mut resume = self.resume_signal.borrow().read().clone();
        self.use_case.update_skills(&mut resume, skills);
        self.resume_signal.borrow_mut().set(resume);
    }
    
    pub fn add_project(&self, project: Project) {
        let mut resume = self.resume_signal.borrow().read().clone();
        self.use_case.add_project(&mut resume, project);
        self.resume_signal.borrow_mut().set(resume);
    }
    
    pub fn update_project(&self, index: usize, project: Project) {
        let mut resume = self.resume_signal.borrow().read().clone();
        if self.use_case.update_project(&mut resume, index, project).is_ok() {
            self.resume_signal.borrow_mut().set(resume);
        }
    }
    
    pub fn remove_project(&self, index: usize) {
        let mut resume = self.resume_signal.borrow().read().clone();
        if self.use_case.remove_project(&mut resume, index).is_ok() {
            self.resume_signal.borrow_mut().set(resume);
        }
    }
    
    pub fn change_theme(&self, theme: ResumeTheme) {
        let mut resume = self.resume_signal.borrow().read().clone();
        self.use_case.change_theme(&mut resume, theme);
        self.resume_signal.borrow_mut().set(resume);
    }
} 