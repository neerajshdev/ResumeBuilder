// Components module - contains all UI components
pub mod personal_info_form;
pub mod resume_preview;
pub mod education_form;
pub mod experience_form;
pub mod theme_selector;
pub mod draggable_section;
pub mod toggle_button;
pub mod export_modal;
pub mod skills_form;
pub mod projects_form;

// Re-exports for convenience
pub use education_form::EducationForm;
pub use experience_form::ExperienceForm;
pub use theme_selector::{Theme, ThemeSelector};
pub use draggable_section::DraggableSection;
pub use toggle_button::ToggleButton;
pub use export_modal::ExportModal;
pub use personal_info_form::PersonalInfoForm;
pub use resume_preview::ResumePreview;
pub use skills_form::SkillsForm;
pub use projects_form::ProjectsForm;
