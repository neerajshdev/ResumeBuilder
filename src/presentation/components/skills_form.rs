use dioxus::prelude::*;
use crate::domain::models::Skill;

/// Component for adding, editing, and removing skills in the resume
#[component]
pub fn SkillsForm(
    skills: Vec<Skill>,
    on_add: EventHandler<Skill>,
    on_remove: EventHandler<usize>,
    on_edit: EventHandler<(usize, Skill)>,
) -> Element {
    // State for form inputs
    let mut skill_name = use_signal(|| String::new());
    let mut skill_level = use_signal(|| 0);
    let mut edit_index = use_signal(|| Option::<usize>::None);
    
    // Clone skills for use in closures
    let skills = skills.clone();
    let skills_for_edit = skills.clone();
    
    // Function to handle form submission
    let handle_submit = move |_| {
        // Validation: name should not be empty
        if skill_name.read().trim().is_empty() {
            return;
        }
        
        let new_skill = Skill {
            name: skill_name.read().clone(),
            level: *skill_level.read(),
        };
        
        // Store index before clearing it
        let current_index = *edit_index.read();
        edit_index.set(None);
        
        if let Some(index) = current_index {
            on_edit.call((index, new_skill));
        } else {
            on_add.call(new_skill);
        }
        
        // Reset form
        skill_name.set(String::new());
        skill_level.set(0);
    };
    
    // Function to handle editing an existing skill
    let handle_edit = EventHandler::new(move |index: usize| {
        let skill = &skills_for_edit[index];
        skill_name.set(skill.name.clone());
        skill_level.set(skill.level);
        edit_index.set(Some(index));
    });
    
    // Function to handle canceling an edit
    let handle_cancel = move |_| {
        skill_name.set(String::new());
        skill_level.set(0);
        edit_index.set(None);
    };
    
    rsx! {
        div { class: "grid grid-cols-1 md:grid-cols-2 gap-6 p-4 bg-white rounded-lg shadow-md",
            // Skills form (left column)
            div { class: "space-y-4",
                h2 { class: "text-xl font-semibold text-gray-800", "Add Skills" }
                
                div { class: "flex flex-col space-y-1",
                    label { class: "text-sm font-medium text-gray-700", "Skill Name" }
                    input {
                        class: "p-2 border rounded-md",
                        placeholder: "e.g. JavaScript, Project Management, etc.",
                        value: "{skill_name}",
                        oninput: move |evt| skill_name.set(evt.value().clone())
                    }
                }
                
                div { class: "flex flex-col space-y-1",
                    label { class: "text-sm font-medium text-gray-700", "Proficiency Level (0-5)" }
                    input {
                        r#type: "range",
                        class: "w-full",
                        min: "0",
                        max: "5",
                        value: "{skill_level}",
                        oninput: move |evt| {
                            if let Ok(level) = evt.value().parse::<i32>() {
                                skill_level.set(level);
                            }
                        }
                    }
                    div { class: "text-center font-medium", "{skill_level} / 5" }
                }
                
                div { class: "flex space-x-2 mt-4",
                    button {
                        class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700",
                        onclick: handle_submit,
                        if edit_index.read().is_some() {
                            "Update Skill"
                        } else {
                            "Add Skill"
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
            
            // Skills list (right column)
            div { class: "space-y-4",
                h2 { class: "text-xl font-semibold text-gray-800", "Skills List" }
                
                if skills.is_empty() {
                    div { class: "text-sm text-gray-500 italic", "No skills added yet." }
                } else {
                    div { class: "space-y-3 max-h-96 overflow-y-auto",
                        for (i, skill) in skills.iter().enumerate() {
                            div {
                                key: "{i}",
                                class: "flex items-center justify-between p-3 bg-gray-50 rounded-md",
                                div { class: "flex-1",
                                    div { class: "font-medium", "{skill.name}" }
                                    div { class: "text-sm text-gray-500", 
                                        "Proficiency: ", 
                                        for j in 0..5 {
                                            span {
                                                key: "{j}",
                                                class: if j < skill.level { "text-yellow-500" } else { "text-gray-300" },
                                                "★"
                                            }
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
                        }
                    }
                }
            }
        }
    }
} 