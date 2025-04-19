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
    let view_model_clone = view_model.clone();
    let view_model_effect_clone = view_model.clone();
    
    // Load existing data if available using an effect to prevent signal write during render
    use_effect(move || {
        let vm = view_model_effect_clone.clone();
        vm.load();
    });
    
    // UI state - using imported Tab from components
    let mut current_tab = use_signal(|| tab::Tab::Personal);
    let mut show_export_modal = use_signal(|| false);
    
    // Define available themes
    let themes = vec![
        ("Professional", "bg-blue-50"),
        ("Minimal", "bg-gray-50"),
        ("Creative", "bg-purple-50"),
        ("Modern", "bg-teal-50"),
        ("Executive", "bg-amber-50"),
        ("Technical", "bg-cyan-50"),
    ];
    let themes_clone = themes.clone();
    
    // Selected theme state
    let mut selected_theme = use_signal(|| 0);
    
    // Function to export resume to PDF (simulated for now)
    let export_to_pdf = move |_| {
        show_export_modal.set(true);
        
        // In a real app, this would convert the resume to PDF
        // and provide a download link to the user
        let theme_name = themes_clone[selected_theme()].0.to_string();
        println!("Exporting resume to PDF with theme: {}", theme_name);
    };
    
    rsx! {
        div {
            class: "min-h-screen bg-gray-100",
            div {
                class: "container mx-auto p-4",
                // Header with logo and actions
                div {
                    class: "flex justify-between items-center mb-6",
                    h1 {
                        class: "text-3xl font-bold text-blue-800",
                        "Resume Builder"
                    },
                    
                    div {
                        class: "flex space-x-2",
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
                            onclick: move |_| { view_model_clone.clone().save(); },
                            {"Save"}
                        }
                    }
                },
                
                // Tab navigation
                TabBar {
                    current_tab: current_tab(),
                    on_tab_change: move |tab| current_tab.set(tab),
                },
                
                // Tab content
                div {
                    class: "p-6 border rounded bg-white shadow-md",
                    match current_tab() {
                        tab::Tab::Personal => rsx! {
                            PersonalInfoForm {
                                personal_info: view_model.clone().resume().personal_info,
                                on_change: move |info| view_model.clone().update_personal_info(info),
                            }
                        },
                        tab::Tab::Education => rsx! {
                            div {
                                h2 {
                                    class: "text-xl font-bold mb-4",
                                    "Education"
                                },
                                
                                // Education form here
                                // In a real app, you would create an EducationForm component
                                p {
                                    class: "text-gray-600",
                                    "Add your educational background here"
                                },
                                
                                // Display existing education entries
                                if !view_model.clone().resume().education.is_empty() {
                                    div {
                                        class: "mt-4 space-y-4",
                                        for (_index, edu) in view_model.clone().resume().education.iter().enumerate() {
                                            div {
                                                class: "p-4 border rounded bg-gray-50",
                                                div {
                                                    class: "font-bold text-lg",
                                                    "{edu.institution}"
                                                },
                                                div {
                                                    "{edu.degree} in {edu.field_of_study}"
                                                },
                                                div {
                                                    class: "text-sm text-gray-600",
                                                    "{edu.start_date} - {edu.end_date}"
                                                },
                                                // Edit and Delete buttons would go here
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        tab::Tab::Experience => rsx! {
                            div {
                                h2 {
                                    class: "text-xl font-bold mb-4",
                                    "Work Experience"
                                },
                                
                                // Experience form here
                                // In a real app, you would create an ExperienceForm component
                                p {
                                    class: "text-gray-600",
                                    "Add your work experience here"
                                },
                                
                                // Display existing experience entries
                                if !view_model.clone().resume().experience.is_empty() {
                                    div {
                                        class: "mt-4 space-y-4",
                                        for (_index, exp) in view_model.clone().resume().experience.iter().enumerate() {
                                            div {
                                                class: "p-4 border rounded bg-gray-50",
                                                div {
                                                    class: "font-bold text-lg",
                                                    "{exp.company}"
                                                },
                                                div {
                                                    "{exp.position}"
                                                },
                                                div {
                                                    class: "text-sm text-gray-600",
                                                    if exp.is_current {
                                                        "{exp.start_date} - Present"
                                                    } else {
                                                        "{exp.start_date} - {exp.end_date}"
                                                    }
                                                },
                                                // Edit and Delete buttons would go here
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        tab::Tab::Skills => rsx! {
                            div {
                                h2 {
                                    class: "text-xl font-bold mb-4",
                                    "Skills"
                                },
                                
                                // Skills form here
                                // In a real app, you would create a SkillsForm component
                                p {
                                    class: "text-gray-600",
                                    "Add your skills here"
                                },
                                
                                // Display existing skill categories
                                if !view_model.clone().resume().skills.categories.is_empty() {
                                    div {
                                        class: "mt-4 space-y-4",
                                        for (category, skills) in view_model.clone().resume().skills.categories.iter() {
                                            div {
                                                class: "p-4 border rounded bg-gray-50",
                                                div {
                                                    class: "font-bold",
                                                    "{category}"
                                                },
                                                div {
                                                    class: "flex flex-wrap gap-2 mt-2",
                                                    for skill in skills {
                                                        span {
                                                            class: "bg-blue-100 px-2 py-1 rounded text-sm",
                                                            "{skill}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        tab::Tab::Projects => rsx! {
                            div {
                                h2 {
                                    class: "text-xl font-bold mb-4",
                                    "Projects"
                                },
                                
                                // Projects form here
                                // In a real app, you would create a ProjectsForm component
                                p {
                                    class: "text-gray-600",
                                    "Add your projects here"
                                },
                                
                                // Display existing projects
                                if !view_model.clone().resume().projects.is_empty() {
                                    div {
                                        class: "mt-4 space-y-4",
                                        for (_index, project) in view_model.clone().resume().projects.iter().enumerate() {
                                            div {
                                                class: "p-4 border rounded bg-gray-50",
                                                div {
                                                    class: "font-bold text-lg",
                                                    "{project.name}"
                                                },
                                                p {
                                                    class: "text-sm",
                                                    "{project.description}"
                                                },
                                                
                                                if !project.technologies.is_empty() {
                                                    div {
                                                        class: "flex flex-wrap gap-2 mt-2",
                                                        for tech in project.technologies.iter() {
                                                            span {
                                                                class: "bg-blue-100 px-2 py-1 rounded text-sm",
                                                                "{tech}"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        tab::Tab::Preview => rsx! {
                            div {
                                // Theme selector
                                div {
                                    class: "mb-6",
                                    h2 {
                                        class: "text-xl font-bold mb-3",
                                        "Resume Preview"
                                    },
                                    
                                    // Theme selection
                                    div {
                                        class: "mb-4",
                                        label {
                                            class: "block text-sm font-medium text-gray-700 mb-2",
                                            "Select Theme:"
                                        },
                                        div {
                                            class: "grid grid-cols-3 gap-2",
                                            for (index, (theme_name, theme_bg)) in themes.iter().enumerate() {
                                                div {
                                                    class: "relative cursor-pointer",
                                                    onclick: move |_| selected_theme.set(index),
                                                    div {
                                                        class: format!("border-2 {} p-2 rounded transition-colors {}", if selected_theme() == index { "border-blue-500" } else { "border-gray-200" }, theme_bg),
                                                        div {
                                                            class: "h-10 flex items-center justify-center",
                                                            "{theme_name}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                // Resume preview with selected theme
                                div {
                                    class: format!("border p-6 bg-white shadow rounded {}", themes[selected_theme()].1),
                                    // Personal info
                                    div {
                                        class: "mb-6 border-b pb-4",
                                        h1 {
                                            class: "text-2xl font-bold",
                                            "{view_model.clone().resume().personal_info.name}"
                                        },
                                        div {
                                            class: "flex flex-wrap gap-2 text-sm text-gray-600",
                                            if !view_model.clone().resume().personal_info.email.is_empty() {
                                                span {
                                                    "{view_model.clone().resume().personal_info.email}"
                                                }
                                            },
                                            if !view_model.clone().resume().personal_info.phone.is_empty() {
                                                span {
                                                    "{view_model.clone().resume().personal_info.phone}"
                                                }
                                            },
                                            if !view_model.clone().resume().personal_info.location.is_empty() {
                                                span {
                                                    "{view_model.clone().resume().personal_info.location}"
                                                }
                                            }
                                        }
                                    },
                                    
                                    // Summary
                                    if !view_model.clone().resume().personal_info.summary.is_empty() {
                                        div {
                                            class: "mt-4",
                                            h2 {
                                                class: "text-lg font-bold border-b",
                                                "Summary"
                                            },
                                            p {
                                                class: "mt-2",
                                                "{view_model.clone().resume().personal_info.summary}"
                                            }
                                        }
                                    },
                                    
                                    // Education
                                    if !view_model.clone().resume().education.is_empty() {
                                        div {
                                            class: "mt-6",
                                            h2 {
                                                class: "text-lg font-bold border-b mb-2",
                                                "Education"
                                            },
                                            for edu in view_model.clone().resume().education.iter() {
                                                div {
                                                    class: "mt-3",
                                                    div {
                                                        class: "font-bold",
                                                        "{edu.institution}"
                                                    },
                                                    div {
                                                        "{edu.degree} in {edu.field_of_study}"
                                                    },
                                                    div {
                                                        class: "text-sm text-gray-600",
                                                        "{edu.start_date} - {edu.end_date}"
                                                    },
                                                    if !edu.description.is_empty() {
                                                        p {
                                                            class: "text-sm mt-1",
                                                            "{edu.description}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    
                                    // Experience
                                    if !view_model.clone().resume().experience.is_empty() {
                                        div {
                                            class: "mt-6",
                                            h2 {
                                                class: "text-lg font-bold border-b mb-2",
                                                "Experience"
                                            },
                                            for exp in view_model.clone().resume().experience.iter() {
                                                div {
                                                    class: "mt-3",
                                                    div {
                                                        class: "font-bold",
                                                        "{exp.company}"
                                                    },
                                                    div {
                                                        "{exp.position}"
                                                    },
                                                    div {
                                                        class: "text-sm text-gray-600",
                                                        if exp.is_current {
                                                            "{exp.start_date} - Present"
                                                        } else {
                                                            "{exp.start_date} - {exp.end_date}"
                                                        }
                                                    },
                                                    p {
                                                        class: "text-sm mt-1",
                                                        "{exp.description}"
                                                    },
                                                    
                                                    if !exp.achievements.is_empty() {
                                                        ul {
                                                            class: "list-disc ml-5 text-sm mt-1",
                                                            for achievement in exp.achievements.iter() {
                                                                li {
                                                                    "{achievement}"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    
                                    // Skills
                                    if !view_model.clone().resume().skills.categories.is_empty() {
                                        div {
                                            class: "mt-6",
                                            h2 {
                                                class: "text-lg font-bold border-b mb-2",
                                                "Skills"
                                            },
                                            for (category, skills) in view_model.clone().resume().skills.categories.iter() {
                                                div {
                                                    class: "mt-2",
                                                    div {
                                                        class: "font-bold",
                                                        "{category}"
                                                    },
                                                    div {
                                                        class: "flex flex-wrap gap-2 mt-1",
                                                        for skill in skills {
                                                            span {
                                                                class: "bg-gray-200 px-2 py-1 rounded text-sm",
                                                                "{skill}"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    
                                    // Projects
                                    if !view_model.clone().resume().projects.is_empty() {
                                        div {
                                            class: "mt-6",
                                            h2 {
                                                class: "text-lg font-bold border-b mb-2",
                                                "Projects"
                                            },
                                            for project in view_model.clone().resume().projects.iter() {
                                                div {
                                                    class: "mt-3",
                                                    div {
                                                        class: "font-bold",
                                                        "{project.name}"
                                                    },
                                                    div {
                                                        class: "text-sm text-gray-600",
                                                        "{project.start_date} - {project.end_date}"
                                                    },
                                                    if !project.url.is_empty() {
                                                        a {
                                                            class: "text-blue-500 text-sm block",
                                                            href: "{project.url}",
                                                            target: "_blank",
                                                            "{project.url}"
                                                        }
                                                    },
                                                    p {
                                                        class: "text-sm mt-1",
                                                        "{project.description}"
                                                    },
                                                    
                                                    if !project.technologies.is_empty() {
                                                        div {
                                                            class: "flex flex-wrap gap-2 mt-2",
                                                            for tech in project.technologies.iter() {
                                                                span {
                                                                    class: "bg-gray-200 px-2 py-1 rounded text-sm",
                                                                    "{tech}"
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
            },
            // Export modal
            if show_export_modal() {
                div {
                    class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                    div {
                        class: "bg-white rounded-lg p-6 w-96 shadow-xl",
                        h3 {
                            class: "text-xl font-bold mb-4",
                            "Export Resume"
                        },
                        p {
                            class: "mb-4",
                            "Your resume has been prepared for export with the '{themes[selected_theme()].0}' theme."
                        },
                        div {
                            class: "flex justify-between",
                            button {
                                class: "px-4 py-2 bg-gray-300 rounded hover:bg-gray-400 transition-colors",
                                onclick: move |_| show_export_modal.set(false),
                                "Close"
                            },
                            button {
                                class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors",
                                onclick: move |_| {
                                    // Simulate download in a real app
                                    println!("Downloading PDF...");
                                    show_export_modal.set(false);
                                },
                                "Download PDF"
                            }
                        }
                    }
                }
            }
        }
    }
}

