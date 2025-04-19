use crate::domain::{Resume, PersonalInfo, Education, Experience, Project, Skills, ResumeTheme};
use crate::application::repository::ResumeRepository;
use std::error::Error;
use std::rc::Rc;

pub struct ResumeUseCase {
    repository: Rc<dyn ResumeRepository>,
}

impl ResumeUseCase {
    pub fn new(repository: Rc<dyn ResumeRepository>) -> Self {
        Self { repository }
    }
    
    pub fn create_new_resume(&self) -> Resume {
        Resume::default()
    }
    
    pub fn save_resume(&self, resume: &Resume) -> Result<(), Box<dyn Error>> {
        self.repository.save(resume)
    }
    
    pub fn load_resume(&self) -> Result<Resume, Box<dyn Error>> {
        if self.repository.exists() {
            self.repository.load()
        } else {
            Ok(Resume::default())
        }
    }
    
    pub fn update_personal_info(&self, resume: &mut Resume, info: PersonalInfo) {
        resume.personal_info = info;
    }
    
    pub fn add_education(&self, resume: &mut Resume, education: Education) {
        resume.education.push(education);
    }
    
    pub fn update_education(&self, resume: &mut Resume, index: usize, education: Education) -> Result<(), &'static str> {
        if index < resume.education.len() {
            resume.education[index] = education;
            Ok(())
        } else {
            Err("Education index out of bounds")
        }
    }
    
    pub fn remove_education(&self, resume: &mut Resume, index: usize) -> Result<(), &'static str> {
        if index < resume.education.len() {
            resume.education.remove(index);
            Ok(())
        } else {
            Err("Education index out of bounds")
        }
    }
    
    pub fn add_experience(&self, resume: &mut Resume, experience: Experience) {
        resume.experience.push(experience);
    }
    
    pub fn update_experience(&self, resume: &mut Resume, index: usize, experience: Experience) -> Result<(), &'static str> {
        if index < resume.experience.len() {
            resume.experience[index] = experience;
            Ok(())
        } else {
            Err("Experience index out of bounds")
        }
    }
    
    pub fn remove_experience(&self, resume: &mut Resume, index: usize) -> Result<(), &'static str> {
        if index < resume.experience.len() {
            resume.experience.remove(index);
            Ok(())
        } else {
            Err("Experience index out of bounds")
        }
    }
    
    pub fn update_skills(&self, resume: &mut Resume, skills: Skills) {
        resume.skills = skills;
    }
    
    pub fn add_project(&self, resume: &mut Resume, project: Project) {
        resume.projects.push(project);
    }
    
    pub fn update_project(&self, resume: &mut Resume, index: usize, project: Project) -> Result<(), &'static str> {
        if index < resume.projects.len() {
            resume.projects[index] = project;
            Ok(())
        } else {
            Err("Project index out of bounds")
        }
    }
    
    pub fn remove_project(&self, resume: &mut Resume, index: usize) -> Result<(), &'static str> {
        if index < resume.projects.len() {
            resume.projects.remove(index);
            Ok(())
        } else {
            Err("Project index out of bounds")
        }
    }
    
    pub fn change_theme(&self, resume: &mut Resume, theme: ResumeTheme) {
        resume.theme = theme;
    }
} 