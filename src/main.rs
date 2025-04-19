mod domain;
mod application;
mod infrastructure;
mod presentation;

use dioxus::prelude::*;
use std::rc::Rc;

use domain::*;
use application::*;
use infrastructure::*;
use presentation::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    // --- State Management ---
    // UI state
    let mut is_preview_mode = use_signal(|| false);
    let mut show_export_modal = use_signal(|| false);
    let mut sections_order = use_signal(|| vec![
        "personal", "education", "experience", "skills", "projects"
    ]);
    
    // Resume data state
    let mut resume = use_signal(|| Resume::default());
    let mut selected_theme = use_signal(|| 0);
    
    // --- Setup Repository and Use Cases ---
    #[cfg(feature = "web")]
    let repository = Rc::new(LocalStorageResumeRepository::new("resume-data"));
    
    #[cfg(not(feature = "web"))]
    let repository = Rc::new(InMemoryResumeRepository::new(use_signal(|| None::<Resume>)));
    
    let use_case = Rc::new(ResumeUseCase::new(repository));
    let use_case_load = use_case.clone();
    let use_case_save = use_case.clone();
    
    // --- Themes ---
    let themes: Vec<Theme> = vec![
        ("Professional", "bg-blue-50"),
        ("Minimal", "bg-gray-50"),
        ("Creative", "bg-purple-50"),
        ("Modern", "bg-teal-50"),
        ("Executive", "bg-amber-50"),
        ("Technical", "bg-cyan-50"),
    ];
    
    // --- Effects ---
    // Load existing data if available
    use_effect(move || {
        if let Ok(loaded_resume) = use_case_load.load_resume() {
            resume.set(loaded_resume);
        }
    });
    
    // --- Event Handlers ---
    // Function to handle section drag
    let mut handle_section_drag = move |from: usize, to: usize| {
        let mut new_order = sections_order();
        let section = new_order.remove(from);
        new_order.insert(to, section);
        sections_order.set(new_order);
    };
    
    // Save resume function
    let save_resume = move |_: Event<MouseData>| {
        if let Err(err) = use_case_save.save_resume(&resume()) {
            println!("Error saving resume: {}", err);
        } else {
            println!("Resume saved successfully");
        }
    };
    
    // Export to PDF function
    let export_to_pdf = move |_: Event<MouseData>| {
        show_export_modal.set(true);
    };
    
    // Function for handling close of export modal
    let close_export_modal = move |_| {
        show_export_modal.set(false);
    };
    
    // Function for handling PDF download
    let download_pdf = move |_| {
        println!("Downloading PDF...");
        show_export_modal.set(false);
    };
    
    // --- Render UI ---
    rsx! {
        div {
            class: "min-h-screen bg-gray-100",
            div {
                class: "container mx-auto p-4 max-w-5xl",
                // Header with title, mode toggle, and actions
                div {
                    class: "flex justify-between items-center mb-6",
                    h1 {
                        class: "text-3xl font-bold text-blue-800",
                        "Resume Builder"
                    },
                    
                    div {
                        class: "flex items-center gap-4",
                        // Edit/Preview toggle
                        ToggleButton {
                            is_preview_mode: is_preview_mode(),
                            on_toggle: move |preview| is_preview_mode.set(preview)
                        },
                        
                        button {
                            class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 transition-colors duration-300 flex items-center",
                            onclick: export_to_pdf,
                            // PDF icon
                            svg {
                                class: "w-5 h-5 mr-2",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                                }
                            },
                            "Export to PDF"
                        },
                        
                        button {
                            class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors duration-300",
                            onclick: save_resume,
                            "Save"
                        }
                    }
                },
                
                // Content - Either Preview or Edit mode
                if is_preview_mode() {
                    // Preview mode
                    ThemeSelector {
                        themes: themes.clone(),
                        selected_theme: selected_theme(),
                        on_theme_select: move |index| selected_theme.set(index)
                    }
                    
                    ResumePreview {
                        resume: resume(),
                        theme_bg: themes[selected_theme()].1
                    }
                } else {
                    // Edit mode - Draggable sections
                    div {
                        class: "space-y-4",
                        p {
                            class: "text-gray-700 italic mb-4",
                            "Tip: Drag and drop sections to reorder them in your resume"
                        },
                        
                        // Render each section in the user-defined order
                        for (index, section_id) in sections_order().iter().enumerate() {
                            DraggableSection {
                                index: index,
                                total_sections: sections_order().len(),
                                on_move_up: if index > 0 {
                                    Some(EventHandler::new(move |_| handle_section_drag(index, index - 1)))
                                } else {
                                    None
                                },
                                on_move_down: if index < sections_order().len() - 1 {
                                    Some(EventHandler::new(move |_| handle_section_drag(index, index + 1)))
                                } else {
                                    None
                                },
                                match *section_id {
                                    "personal" => rsx! {
                                        PersonalInfoForm {
                                            personal_info: resume().personal_info,
                                            on_change: move |info| {
                                                let mut updated_resume = resume();
                                                updated_resume.personal_info = info;
                                                resume.set(updated_resume);
                                            }
                                        }
                                    },
                                    "education" => rsx! {
                                        div {
                                            h2 {
                                                class: "text-xl font-bold mb-4",
                                                "Education"
                                            },
                                            
                                            EducationForm {
                                                education_list: resume().education.clone(),
                                                on_add: move |edu| {
                                                    let mut updated_resume = resume();
                                                    updated_resume.education.push(edu);
                                                    resume.set(updated_resume);
                                                },
                                                on_update: move |(index, edu)| {
                                                    let mut updated_resume = resume();
                                                    if let Some(existing_edu) = updated_resume.education.get_mut(index) {
                                                        *existing_edu = edu;
                                                    }
                                                    resume.set(updated_resume);
                                                },
                                                on_remove: move |index| {
                                                    let mut updated_resume = resume();
                                                    updated_resume.education.remove(index);
                                                    resume.set(updated_resume);
                                                },
                                                on_edit: move |_index| {
                                                    // Handled within EducationForm
                                                }
                                            }
                                        }
                                    },
                                    "experience" => rsx! {
                                        div {
                                            h2 {
                                                class: "text-xl font-bold mb-4",
                                                "Work Experience"
                                            },
                                            
                                            ExperienceForm {
                                                experience_list: resume().experience.clone(),
                                                on_add: move |exp| {
                                                    let mut updated_resume = resume();
                                                    updated_resume.experience.push(exp);
                                                    resume.set(updated_resume);
                                                },
                                                on_update: move |(index, exp)| {
                                                    let mut updated_resume = resume();
                                                    if let Some(existing_exp) = updated_resume.experience.get_mut(index) {
                                                        *existing_exp = exp;
                                                    }
                                                    resume.set(updated_resume);
                                                },
                                                on_remove: move |index| {
                                                    let mut updated_resume = resume();
                                                    updated_resume.experience.remove(index);
                                                    resume.set(updated_resume);
                                                },
                                                on_edit: move |_index| {
                                                    // Handled within ExperienceForm
                                                }
                                            }
                                        }
                                    },
                                    "skills" => rsx! {
                                        div {
                                            h2 {
                                                class: "text-xl font-bold mb-4",
                                                "Skills"
                                            },
                                            
                                            // Skills form placeholder
                                            p {
                                                class: "text-gray-600",
                                                "Skills section coming soon"
                                            }
                                        }
                                    },
                                    "projects" => rsx! {
                                        div {
                                            h2 {
                                                class: "text-xl font-bold mb-4",
                                                "Projects"
                                            },
                                            
                                            // Projects form placeholder
                                            p {
                                                class: "text-gray-600",
                                                "Projects section coming soon"
                                            }
                                        }
                                    },
                                    _ => rsx! { div { "Unknown section" } }
                                }
                            }
                        }
                    }
                }
            },
            
            // Export modal
            ExportModal {
                show: show_export_modal(),
                theme_name: themes[selected_theme()].0.to_string(),
                on_close: EventHandler::new(close_export_modal),
                on_download: EventHandler::new(download_pdf)
            }
        }
    }
}

