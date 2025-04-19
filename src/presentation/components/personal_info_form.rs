use dioxus::prelude::*;
use crate::domain::models::PersonalInfo;

#[component]
pub fn PersonalInfoForm(
    personal_info: PersonalInfo,
    on_change: EventHandler<PersonalInfo>,
) -> Element {
    let mut name = use_signal(|| personal_info.name.clone());
    let mut email = use_signal(|| personal_info.email.clone());
    let mut phone = use_signal(|| personal_info.phone.clone());
    let mut location = use_signal(|| personal_info.location.clone());
    let mut summary = use_signal(|| personal_info.summary.clone());
    
    // Handle optional social media fields
    let mut github = use_signal(|| personal_info.github.clone());
    let mut linkedin = use_signal(|| personal_info.linkedin.clone());
    let mut website = use_signal(|| personal_info.website.clone());

    // Update the parent component when any field changes
    let update_parent = move || {
        let updated_info = PersonalInfo {
            name: name().clone(),
            email: email().clone(),
            phone: phone().clone(),
            location: location().clone(),
            summary: summary().clone(),
            github: github().clone(),
            linkedin: linkedin().clone(),
            website: website().clone(),
        };
        on_change.call(updated_info);
    };

    rsx! {
        div {
            h2 {
                class: "text-xl font-bold mb-4",
                "Personal Information"
            },
            div {
                class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 mb-1",
                        "Full Name"
                    },
                    input {
                        class: "w-full p-2 border rounded",
                        value: "{name}",
                        oninput: move |evt| {
                            name.set(evt.value());
                            update_parent();
                        }
                    }
                },
                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 mb-1",
                        "Email"
                    },
                    input {
                        class: "w-full p-2 border rounded",
                        value: "{email}",
                        oninput: move |evt| {
                            email.set(evt.value());
                            update_parent();
                        }
                    }
                },
                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 mb-1",
                        "Phone"
                    },
                    input {
                        class: "w-full p-2 border rounded",
                        value: "{phone}",
                        oninput: move |evt| {
                            phone.set(evt.value());
                            update_parent();
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
                        value: "{location}",
                        oninput: move |evt| {
                            location.set(evt.value());
                            update_parent();
                        }
                    }
                }
            },
            
            // Social media links
            div {
                class: "grid grid-cols-1 md:grid-cols-3 gap-4 mt-4",
                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 mb-1",
                        "GitHub"
                    },
                    input {
                        class: "w-full p-2 border rounded",
                        value: "{github}",
                        placeholder: "github.com/username",
                        oninput: move |evt| {
                            github.set(evt.value());
                            update_parent();
                        }
                    }
                },
                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 mb-1",
                        "LinkedIn"
                    },
                    input {
                        class: "w-full p-2 border rounded",
                        value: "{linkedin}",
                        placeholder: "linkedin.com/in/username",
                        oninput: move |evt| {
                            linkedin.set(evt.value());
                            update_parent();
                        }
                    }
                },
                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 mb-1",
                        "Personal Website"
                    },
                    input {
                        class: "w-full p-2 border rounded",
                        value: "{website}",
                        placeholder: "example.com",
                        oninput: move |evt| {
                            website.set(evt.value());
                            update_parent();
                        }
                    }
                }
            },
            
            div {
                class: "mt-4",
                label {
                    class: "block text-sm font-medium text-gray-700 mb-1",
                    "Professional Summary"
                },
                textarea {
                    class: "w-full p-2 border rounded",
                    rows: "5",
                    value: "{summary}",
                    oninput: move |evt| {
                        summary.set(evt.value());
                        update_parent();
                    }
                }
            }
        }
    }
} 