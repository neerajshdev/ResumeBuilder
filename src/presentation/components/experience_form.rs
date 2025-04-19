use dioxus::prelude::*;
use crate::domain::models::Experience;

#[component]
pub fn ExperienceForm(
    experience_list: Vec<Experience>,
    on_add: EventHandler<Experience>,
    on_update: EventHandler<(usize, Experience)>,
    on_remove: EventHandler<usize>,
    on_edit: EventHandler<usize>
) -> Element {
    let experience_list_clone = experience_list.clone();
    let mut new_company = use_signal(|| String::new());
    let mut new_position = use_signal(|| String::new());
    let mut new_start_date = use_signal(|| String::new());
    let mut new_end_date = use_signal(|| String::new());
    let mut new_location = use_signal(|| String::new());
    let mut new_is_current = use_signal(|| false);
    let mut new_description = use_signal(|| String::new());
    let mut new_achievements = use_signal(|| String::new());
    let mut editing_index = use_signal(|| None::<usize>);

    let handle_submit = move |_| {
        let achievements: Vec<String> = new_achievements()
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        let experience = Experience {
            company: new_company().clone(),
            position: new_position().clone(),
            start_date: new_start_date().clone(),
            end_date: new_end_date().clone(),
            is_current: new_is_current(),
            location: new_location().clone(),
            description: new_description().clone(),
            achievements,
        };

        if let Some(index) = editing_index() {
            on_update.call((index, experience));
            editing_index.set(None);
        } else {
            on_add.call(experience);
        }

        // Clear form fields
        new_company.set(String::new());
        new_position.set(String::new());
        new_start_date.set(String::new());
        new_end_date.set(String::new());
        new_location.set(String::new());
        new_is_current.set(false);
        new_description.set(String::new());
        new_achievements.set(String::new());
    };

    let _handle_edit = move |index: usize| {
        let exp = &experience_list_clone[index];
        new_company.set(exp.company.clone());
        new_position.set(exp.position.clone());
        new_start_date.set(exp.start_date.clone());
        new_end_date.set(exp.end_date.clone());
        new_location.set(exp.location.clone());
        new_is_current.set(exp.is_current);
        new_description.set(exp.description.clone());
        new_achievements.set(exp.achievements.join("\n"));
        editing_index.set(Some(index));
    };

    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 gap-6",
            // Form section on the left
            div {
                h3 {
                    class: "text-lg font-semibold mb-3",
                    if editing_index().is_some() { "Edit Experience" } else { "Add Experience" }
                },

                div {
                    class: "grid grid-cols-1 gap-4",
                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "Company"
                        },
                        input {
                            class: "w-full p-2 border rounded",
                            value: "{new_company}",
                            oninput: move |event| new_company.set(event.value())
                        }
                    },

                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "Position"
                        },
                        input {
                            class: "w-full p-2 border rounded",
                            value: "{new_position}",
                            oninput: move |event| new_position.set(event.value())
                        }
                    },

                    div {
                        class: "grid grid-cols-2 gap-4",
                        div {
                            label {
                                class: "block text-sm font-medium text-gray-700 mb-1",
                                "Start Date"
                            },
                            input {
                                class: "w-full p-2 border rounded",
                                value: "{new_start_date}",
                                oninput: move |event| new_start_date.set(event.value())
                            }
                        },
                        div {
                            label {
                                class: "block text-sm font-medium text-gray-700 mb-1",
                                "End Date"
                            },
                            input {
                                class: "w-full p-2 border rounded",
                                value: "{new_end_date}",
                                oninput: move |event| new_end_date.set(event.value()),
                                disabled: new_is_current()
                            }
                        }
                    },

                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "Location"
                        },
                        input {
                            class: "w-full p-2 border rounded",
                            value: "{new_location}",
                            oninput: move |event| new_location.set(event.value())
                        }
                    },

                    div {
                        label {
                            class: "flex items-center text-sm font-medium text-gray-700 mb-1",
                            input {
                                r#type: "checkbox",
                                class: "mr-2",
                                checked: new_is_current(),
                                onchange: move |event| new_is_current.set(event.checked())
                            },
                            "Current Position"
                        }
                    },

                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "Description"
                        },
                        textarea {
                            class: "w-full p-2 border rounded",
                            value: "{new_description}",
                            oninput: move |event| new_description.set(event.value())
                        }
                    },

                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "Achievements (one per line)"
                        },
                        textarea {
                            class: "w-full p-2 border rounded",
                            value: "{new_achievements}",
                            oninput: move |event| new_achievements.set(event.value())
                        }
                    }
                },

                button {
                    class: "mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                    onclick: handle_submit,
                    if editing_index().is_some() { "Update" } else { "Add" }
                }
            },

            // Experience entries on the right
            div {
                h4 {
                    class: "text-md font-semibold mb-2",
                    "Experience Entries"
                },
                if !experience_list.is_empty() {
                    div {
                        class: "space-y-4 max-h-[600px] overflow-y-auto pr-2",
                        for (index, exp) in experience_list.iter().enumerate() {
                            div {
                                class: "p-4 border rounded bg-gray-50 shadow-sm hover:shadow transition-shadow",
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
                                if !exp.description.is_empty() {
                                    p {
                                        class: "text-sm mt-1",
                                        "{exp.description}"
                                    }
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
                                },
                                div {
                                    class: "mt-2 flex gap-2",
                                    button {
                                        class: "px-3 py-1 bg-yellow-500 text-white rounded hover:bg-yellow-600",
                                        onclick: move |_| on_edit.call(index),
                                        {"Edit"}
                                    },
                                    button {
                                        class: "px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600",
                                        onclick: move |_| on_remove.call(index),
                                        {"Remove"}
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div {
                        class: "p-4 border rounded bg-gray-50 text-gray-500 italic",
                        "No experience entries yet. Add your first one using the form."
                    }
                }
            }
        }
    }
} 