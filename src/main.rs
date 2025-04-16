mod domain;
mod application;
mod infrastructure;
mod presentation;

use dioxus::prelude::*;
use std::rc::Rc;

use domain::*;
use application::*;
use infrastructure::*;
use presentation::*;
use presentation::components::tab;

fn main() {
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    #[cfg(feature = "web")]
    let repository = Rc::new(LocalStorageResumeRepository::new("resume-data"));
    
    #[cfg(not(feature = "web"))]
    let resume_signal = use_signal(|| None::<Resume>);
    
    #[cfg(not(feature = "web"))]
    let repository = Rc::new(InMemoryResumeRepository::new(resume_signal));
    
    // Initialize use case
    let use_case = Rc::new(ResumeUseCase::new(repository));
    
    // Initialize view model
    let view_model = Rc::new(ResumeViewModel::new(use_case));
    
    // Load existing data if available
    view_model.load();
    
    // UI state - using imported Tab from components
    let mut current_tab = use_signal(|| tab::Tab::Personal);
    
    rsx! {
        div { class: "container mx-auto p-4",
            h1 { class: "text-2xl font-bold text-center mb-4", "Resume Builder" }
            
            // Tab navigation
            TabBar {
                current_tab: current_tab(),
                on_tab_change: move |tab| current_tab.set(tab),
            }
            
            // Tab content
            div { class: "p-4 border rounded",
                match current_tab() {
                    tab::Tab::Personal => rsx! {
                        PersonalInfoForm {
                            personal_info: view_model.resume().personal_info,
                            on_change: move |info| view_model.update_personal_info(info),
                        }
                    },
                    tab::Tab::Education => rsx! {
                        h2 { class: "text-xl font-bold mb-3", "Education" }
                        p { "Education tab content will go here" }
                    },
                    tab::Tab::Experience => rsx! {
                        h2 { class: "text-xl font-bold mb-3", "Experience" }
                        p { "Experience tab content will go here" }
                    },
                    tab::Tab::Skills => rsx! {
                        h2 { class: "text-xl font-bold mb-3", "Skills" }
                        p { "Skills tab content will go here" }
                    },
                    tab::Tab::Projects => rsx! {
                        h2 { class: "text-xl font-bold mb-3", "Projects" }
                        p { "Projects tab content will go here" }
                    },
                    tab::Tab::Preview => {
                        rsx! {
                            h2 { class: "text-xl font-bold mb-3", "Preview" }
                            
                            // Theme selector
                            div { class: "mb-4",
                                label { class: "block mb-2", "Select Theme:" }
                                div { class: "flex space-x-2",
                                    for theme in ResumeTheme::all() {
                                        {
                                            // Create a new clone for each button's closure
                                            let vm = view_model.clone();
                                            let theme = theme;  // Clone theme to move into closure
                                            rsx! {
                                                button { 
                                                    class: if view_model.resume().theme == theme {
                                                        "px-4 py-2 bg-blue-500 text-white rounded"
                                                    } else {
                                                        "px-4 py-2 bg-gray-200 rounded"
                                                    },
                                                    onclick: move |_| vm.change_theme(theme),
                                                    "{theme.name()}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // Save button
                            div { class: "mb-4",
                                {
                                    // Create a new clone for the save button
                                    let vm = view_model.clone();
                                    rsx! {
                                        button {
                                            class: "px-4 py-2 bg-green-500 text-white rounded",
                                            onclick: move |_| vm.save(),
                                            "Save Resume"
                                        }
                                    }
                                }
                            }
                            
                            // Resume preview
                            div { class: "border p-4 bg-white",
                                // Personal info
                                h1 { class: "text-2xl font-bold", "{view_model.resume().personal_info.name}" }
                                div { class: "flex flex-wrap gap-2 text-sm text-gray-600",
                                    span { "{view_model.resume().personal_info.email}" }
                                    span { "{view_model.resume().personal_info.phone}" }
                                    span { "{view_model.resume().personal_info.location}" }
                                }
                                
                                // Summary
                                if !view_model.resume().personal_info.summary.is_empty() {
                                    div { class: "mt-4",
                                        h2 { class: "text-lg font-bold border-b", "Summary" }
                                        p { "{view_model.resume().personal_info.summary}" }
                                    }
                                }
                                
                                // Education
                                if !view_model.resume().education.is_empty() {
                                    div { class: "mt-4",
                                        h2 { class: "text-lg font-bold border-b", "Education" }
                                        for edu in view_model.resume().education.iter() {
                                            div { class: "mt-2",
                                                div { class: "font-bold", "{edu.institution}" }
                                                div { "{edu.degree} in {edu.field_of_study}" }
                                                div { class: "text-sm text-gray-600", "{edu.start_date} - {edu.end_date}" }
                                                if !edu.description.is_empty() {
                                                    p { class: "text-sm", "{edu.description}" }
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                // Experience
                                if !view_model.resume().experience.is_empty() {
                                    div { class: "mt-4",
                                        h2 { class: "text-lg font-bold border-b", "Experience" }
                                        for exp in view_model.resume().experience.iter() {
                                            div { class: "mt-2",
                                                div { class: "font-bold", "{exp.company}" }
                                                div { "{exp.position}" }
                                                div {
                                                    class: "text-sm text-gray-600", 
                                                    if exp.is_current {
                                                        "{exp.start_date} - Present"
                                                    } else {
                                                        "{exp.start_date} - {exp.end_date}"
                                                    }
                                                }
                                                p { class: "text-sm", "{exp.description}" }
                                                
                                                if !exp.achievements.is_empty() {
                                                    ul { class: "list-disc ml-5 text-sm",
                                                        for achievement in exp.achievements.iter() {
                                                            li { "{achievement}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                // Skills
                                if !view_model.resume().skills.categories.is_empty() {
                                    div { class: "mt-4",
                                        h2 { class: "text-lg font-bold border-b", "Skills" }
                                        for (category, skills) in view_model.resume().skills.categories.iter() {
                                            div { class: "mt-2",
                                                div { class: "font-bold", "{category}" }
                                                div { class: "flex flex-wrap gap-2",
                                                    for skill in skills {
                                                        span { class: "bg-gray-200 px-2 py-1 rounded text-sm", "{skill}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                // Projects
                                if !view_model.resume().projects.is_empty() {
                                    div { class: "mt-4",
                                        h2 { class: "text-lg font-bold border-b", "Projects" }
                                        for project in view_model.resume().projects.iter() {
                                            div { class: "mt-2",
                                                div { class: "font-bold", "{project.name}" }
                                                div { class: "text-sm text-gray-600", "{project.start_date} - {project.end_date}" }
                                                if !project.url.is_empty() {
                                                    a { 
                                                        class: "text-blue-500 text-sm", 
                                                        href: "{project.url}", 
                                                        target: "_blank",
                                                        "{project.url}"
                                                    }
                                                }
                                                p { class: "text-sm", "{project.description}" }
                                                
                                                if !project.technologies.is_empty() {
                                                    div { class: "flex flex-wrap gap-2 mt-1",
                                                        for tech in project.technologies.iter() {
                                                            span { class: "bg-gray-200 px-2 py-1 rounded text-sm", "{tech}" }
                                                        }
                                                    }
                                                }
                                                
                                                if !project.achievements.is_empty() {
                                                    ul { class: "list-disc ml-5 text-sm",
                                                        for achievement in project.achievements.iter() {
                                                            li { "{achievement}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Personal,
    Education,
    Experience,
    Skills,
    Projects,
    Preview,
}
