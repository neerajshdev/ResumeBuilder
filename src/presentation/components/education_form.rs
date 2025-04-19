use dioxus::prelude::*;
use crate::domain::models::Education;

#[component]
pub fn EducationForm(
    education_list: Vec<Education>,
    on_add: EventHandler<Education>,
    on_update: EventHandler<(usize, Education)>,
    on_remove: EventHandler<usize>,
    on_edit: EventHandler<usize>
) -> Element {
    let education_list_clone = education_list.clone();
    let mut new_institution = use_signal(|| String::new());
    let mut new_degree = use_signal(|| String::new());
    let mut new_field = use_signal(|| String::new());
    let mut new_start_date = use_signal(|| String::new());
    let mut new_end_date = use_signal(|| String::new());
    let mut new_location = use_signal(|| String::new());
    let mut new_description = use_signal(|| String::new());
    let mut new_gpa = use_signal(|| String::new());
    let mut editing_index = use_signal(|| None::<usize>);

    let handle_submit = move |_| {
        let education = Education {
            institution: new_institution().clone(),
            degree: new_degree().clone(),
            field_of_study: new_field().clone(),
            start_date: new_start_date().clone(),
            end_date: new_end_date().clone(),
            location: new_location().clone(),
            description: new_description().clone(),
            gpa: new_gpa().clone(),
        };

        if let Some(index) = editing_index() {
            on_update.call((index, education));
            editing_index.set(None);
        } else {
            on_add.call(education);
        }

        // Clear form fields
        new_institution.set(String::new());
        new_degree.set(String::new());
        new_field.set(String::new());
        new_start_date.set(String::new());
        new_end_date.set(String::new());
        new_location.set(String::new());
        new_description.set(String::new());
        new_gpa.set(String::new());
    };

    let _handle_edit = move |index: usize| {
        let edu = &education_list_clone[index];
        new_institution.set(edu.institution.clone());
        new_degree.set(edu.degree.clone());
        new_field.set(edu.field_of_study.clone());
        new_start_date.set(edu.start_date.clone());
        new_end_date.set(edu.end_date.clone());
        new_location.set(edu.location.clone());
        new_description.set(edu.description.clone());
        new_gpa.set(edu.gpa.clone());
        editing_index.set(Some(index));
    };

    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 gap-6",
            // Form section on the left
            div {
                h3 {
                    class: "text-lg font-semibold mb-3",
                    if editing_index().is_some() { "Edit Education" } else { "Add Education" }
                },

                div {
                    class: "grid grid-cols-1 gap-4",
                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "Institution"
                        },
                        input {
                            class: "w-full p-2 border rounded",
                            value: "{new_institution}",
                            oninput: move |event| new_institution.set(event.value())
                        }
                    },

                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "Degree"
                        },
                        input {
                            class: "w-full p-2 border rounded",
                            value: "{new_degree}",
                            oninput: move |event| new_degree.set(event.value())
                        }
                    },

                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "Field of Study"
                        },
                        input {
                            class: "w-full p-2 border rounded",
                            value: "{new_field}",
                            oninput: move |event| new_field.set(event.value())
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
                                oninput: move |event| new_end_date.set(event.value())
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
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "GPA"
                        },
                        input {
                            class: "w-full p-2 border rounded",
                            value: "{new_gpa}",
                            oninput: move |event| new_gpa.set(event.value())
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
                    }
                },

                button {
                    class: "mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                    onclick: handle_submit,
                    if editing_index().is_some() { "Update" } else { "Add" }
                }
            },

            // Education entries on the right
            div {
                h4 {
                    class: "text-md font-semibold mb-2",
                    "Education Entries"
                },
                if !education_list.is_empty() {
                    div {
                        class: "space-y-4 max-h-[600px] overflow-y-auto pr-2",
                        for (index, edu) in education_list.iter().enumerate() {
                            div {
                                class: "p-4 border rounded bg-gray-50 shadow-sm hover:shadow transition-shadow",
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
                                if !edu.location.is_empty() {
                                    div {
                                        class: "text-sm text-gray-600",
                                        "{edu.location}"
                                    }
                                },
                                if !edu.gpa.is_empty() {
                                    div {
                                        class: "text-sm text-gray-600",
                                        "GPA: {edu.gpa}"
                                    }
                                },
                                if !edu.description.is_empty() {
                                    p {
                                        class: "text-sm mt-1",
                                        "{edu.description}"
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
                        "No education entries yet. Add your first one using the form."
                    }
                }
            }
        }
    }
} 