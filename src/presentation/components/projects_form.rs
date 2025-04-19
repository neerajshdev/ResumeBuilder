use dioxus::prelude::*;
use crate::domain::models::Project;
use chrono::NaiveDate;

/// Component for adding, editing, and removing projects in the resume
#[component]
pub fn ProjectsForm(
    projects: Vec<Project>,
    on_add: EventHandler<Project>,
    on_remove: EventHandler<usize>,
    on_edit: EventHandler<(usize, Project)>,
) -> Element {
    // State for form inputs
    let mut project_name = use_signal(|| String::new());
    let mut project_description = use_signal(|| String::new());
    let mut project_role = use_signal(|| String::new());
    let mut project_url = use_signal(|| String::new());
    let mut project_technologies = use_signal(|| String::new());
    let mut start_date = use_signal(|| String::new());
    let mut end_date = use_signal(|| String::new());
    let mut edit_index = use_signal(|| Option::<usize>::None);
    
    // Clone projects for use in closures
    let projects = projects.clone();
    let projects_for_edit = projects.clone();
    
    // Function to handle form submission
    let handle_submit = move |_| {
        // Validation: name should not be empty
        if project_name.read().trim().is_empty() {
            return;
        }
        
        // Convert technologies string to vector
        let technologies = project_technologies.read()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>();
        
        let new_project = Project {
            name: project_name.read().clone(),
            description: project_description.read().clone(),
            role: project_role.read().clone(),
            url: project_url.read().clone(),
            start_date: start_date.read().parse::<NaiveDate>().ok(),
            end_date: end_date.read().parse::<NaiveDate>().ok(),
            technologies,
        };
        
        // Store index before clearing it
        let current_index = *edit_index.read();
        edit_index.set(None);
        
        if let Some(index) = current_index {
            on_edit.call((index, new_project));
        } else {
            on_add.call(new_project);
        }
        
        // Reset form
        project_name.set(String::new());
        project_description.set(String::new());
        project_role.set(String::new());
        project_url.set(String::new());
        project_technologies.set(String::new());
        start_date.set(String::new());
        end_date.set(String::new());
    };
    
    // Function to handle editing an existing project
    let handle_edit = EventHandler::new(move |index: usize| {
        let project = &projects_for_edit[index];
        project_name.set(project.name.clone());
        project_description.set(project.description.clone());
        project_role.set(project.role.clone());
        project_url.set(project.url.clone());
        project_technologies.set(project.technologies.join(", "));
        
        // Convert Optional NaiveDate to String in ISO format (YYYY-MM-DD)
        start_date.set(project.start_date
            .map(|date| date.format("%Y-%m-%d").to_string())
            .unwrap_or_default());
            
        end_date.set(project.end_date
            .map(|date| date.format("%Y-%m-%d").to_string())
            .unwrap_or_default());
            
        edit_index.set(Some(index));
    });
    
    // Function to handle canceling an edit
    let handle_cancel = move |_| {
        project_name.set(String::new());
        project_description.set(String::new());
        project_role.set(String::new());
        project_url.set(String::new());
        project_technologies.set(String::new());
        start_date.set(String::new());
        end_date.set(String::new());
        edit_index.set(None);
    };
    
    rsx! {
        div { class: "grid grid-cols-1 md:grid-cols-2 gap-6 p-4 bg-white rounded-lg shadow-md",
            // Projects form (left column)
            div { class: "space-y-4",
                h2 { class: "text-xl font-semibold text-gray-800", "Add Project" }
                
                div { class: "flex flex-col space-y-1",
                    label { class: "text-sm font-medium text-gray-700", "Project Name" }
                    input {
                        class: "p-2 border rounded-md",
                        placeholder: "e.g. Personal Website, Mobile App, etc.",
                        value: "{project_name}",
                        oninput: move |evt| project_name.set(evt.value().clone())
                    }
                }
                
                div { class: "flex flex-col space-y-1",
                    label { class: "text-sm font-medium text-gray-700", "Your Role" }
                    input {
                        class: "p-2 border rounded-md",
                        placeholder: "e.g. Developer, Team Lead, etc.",
                        value: "{project_role}",
                        oninput: move |evt| project_role.set(evt.value().clone())
                    }
                }
                
                div { class: "grid grid-cols-2 gap-4",
                    div { class: "flex flex-col space-y-1",
                        label { class: "text-sm font-medium text-gray-700", "Start Date" }
                        input {
                            r#type: "date",
                            class: "p-2 border rounded-md",
                            value: "{start_date}",
                            oninput: move |evt| start_date.set(evt.value().clone())
                        }
                    }
                    
                    div { class: "flex flex-col space-y-1",
                        label { class: "text-sm font-medium text-gray-700", "End Date" }
                        input {
                            r#type: "date",
                            class: "p-2 border rounded-md",
                            value: "{end_date}",
                            oninput: move |evt| end_date.set(evt.value().clone())
                        }
                    }
                }
                
                div { class: "flex flex-col space-y-1",
                    label { class: "text-sm font-medium text-gray-700", "Project URL" }
                    input {
                        class: "p-2 border rounded-md",
                        placeholder: "e.g. https://github.com/username/project",
                        value: "{project_url}",
                        oninput: move |evt| project_url.set(evt.value().clone())
                    }
                }
                
                div { class: "flex flex-col space-y-1",
                    label { class: "text-sm font-medium text-gray-700", "Technologies (comma separated)" }
                    input {
                        class: "p-2 border rounded-md",
                        placeholder: "e.g. React, Node.js, Docker",
                        value: "{project_technologies}",
                        oninput: move |evt| project_technologies.set(evt.value().clone())
                    }
                }
                
                div { class: "flex flex-col space-y-1",
                    label { class: "text-sm font-medium text-gray-700", "Description" }
                    textarea {
                        class: "p-2 border rounded-md h-32",
                        placeholder: "Describe the project, your contributions, and outcomes...",
                        value: "{project_description}",
                        oninput: move |evt| project_description.set(evt.value().clone())
                    }
                }
                
                div { class: "flex space-x-2 mt-4",
                    button {
                        class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700",
                        onclick: handle_submit,
                        if edit_index.read().is_some() {
                            "Update Project"
                        } else {
                            "Add Project"
                        }
                    }
                    
                    if edit_index.read().is_some() {
                        button {
                            class: "px-4 py-2 bg-gray-300 text-gray-700 rounded-md hover:bg-gray-400",
                            onclick: handle_cancel,
                            "Cancel"
                        }
                    }
                }
            }
            
            // Projects list (right column)
            div { class: "space-y-4",
                h2 { class: "text-xl font-semibold text-gray-800", "Project List" }
                
                if projects.is_empty() {
                    div { class: "text-sm text-gray-500 italic", "No projects added yet." }
                } else {
                    div { class: "space-y-4 max-h-96 overflow-y-auto",
                        for (i, project) in projects.iter().enumerate() {
                            div {
                                key: "{i}",
                                class: "p-4 bg-gray-50 rounded-md",
                                div { class: "flex justify-between items-start",
                                    div { class: "space-y-1",
                                        h3 { class: "font-medium text-lg", "{project.name}" }
                                        div { class: "text-gray-600", "{project.role}" }
                                        div { class: "text-sm text-gray-500",
                                            if let Some(start) = project.start_date {
                                                "{start.format(\"%b %Y\")}"
                                            }
                                            " - "
                                            if let Some(end) = project.end_date {
                                                "{end.format(\"%b %Y\")}"
                                            } else {
                                                "Present"
                                            }
                                        }
                                    }
                                    div { class: "flex space-x-2",
                                        button {
                                            class: "p-1 text-blue-600 hover:text-blue-800",
                                            onclick: move |_| handle_edit.call(i),
                                            "Edit"
                                        }
                                        button {
                                            class: "p-1 text-red-600 hover:text-red-800",
                                            onclick: move |_| on_remove.call(i),
                                            "Remove"
                                        }
                                    }
                                }
                                
                                if !project.description.is_empty() {
                                    div { class: "mt-2 text-sm",
                                        "{project.description}"
                                    }
                                }
                                
                                if !project.technologies.is_empty() {
                                    div { class: "mt-2 flex flex-wrap gap-1",
                                        for tech in project.technologies.iter() {
                                            span {
                                                key: "{tech}",
                                                class: "px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded",
                                                "{tech}"
                                            }
                                        }
                                    }
                                }
                                
                                if !project.url.is_empty() {
                                    div { class: "mt-2 text-sm text-gray-600", 
                                        span { "URL: " }
                                        a {
                                            class: "text-blue-600 hover:underline",
                                            href: "{project.url}",
                                            target: "_blank",
                                            "{project.url}"
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