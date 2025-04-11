use dioxus::prelude::*;
use crate::domain::PersonalInfo;

#[component]
pub fn PersonalInfoForm(
    personal_info: PersonalInfo,
    on_change: EventHandler<PersonalInfo>,
) -> Element {
    let mut info = use_signal(|| personal_info);
    
    let mut handle_change = move |field: &str, value: String| {
        let mut updated = info.read().clone();
        match field {
            "name" => updated.name = value,
            "email" => updated.email = value,
            "phone" => updated.phone = value,
            "website" => updated.website = value,
            "linkedin" => updated.linkedin = value,
            "github" => updated.github = value,
            "location" => updated.location = value,
            "summary" => updated.summary = value,
            _ => {}
        }
        info.set(updated.clone());
        on_change.call(updated);
    };
    
    rsx! {
        div {
            h2 { class: "text-xl font-bold mb-3", "Personal Information" }
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                div {
                    label { class: "block", "Name" }
                    input {
                        class: "w-full p-2 border rounded",
                        r#type: "text",
                        value: "{info().name}",
                        oninput: move |evt| handle_change("name", evt.value())
                    }
                }
                div {
                    label { class: "block", "Email" }
                    input {
                        class: "w-full p-2 border rounded",
                        r#type: "email",
                        value: "{info().email}",
                        oninput: move |evt| handle_change("email", evt.value())
                    }
                }
                div {
                    label { class: "block", "Phone" }
                    input {
                        class: "w-full p-2 border rounded",
                        r#type: "tel",
                        value: "{info().phone}",
                        oninput: move |evt| handle_change("phone", evt.value())
                    }
                }
                div {
                    label { class: "block", "Website" }
                    input {
                        class: "w-full p-2 border rounded",
                        r#type: "url",
                        value: "{info().website}",
                        oninput: move |evt| handle_change("website", evt.value())
                    }
                }
                div {
                    label { class: "block", "LinkedIn" }
                    input {
                        class: "w-full p-2 border rounded",
                        r#type: "url",
                        value: "{info().linkedin}",
                        oninput: move |evt| handle_change("linkedin", evt.value())
                    }
                }
                div {
                    label { class: "block", "GitHub" }
                    input {
                        class: "w-full p-2 border rounded",
                        r#type: "url",
                        value: "{info().github}",
                        oninput: move |evt| handle_change("github", evt.value())
                    }
                }
                div {
                    label { class: "block", "Location" }
                    input {
                        class: "w-full p-2 border rounded",
                        r#type: "text",
                        value: "{info().location}",
                        oninput: move |evt| handle_change("location", evt.value())
                    }
                }
            }
            div { class: "mt-4",
                label { class: "block", "Professional Summary" }
                textarea {
                    class: "w-full p-2 border rounded",
                    rows: "4",
                    value: "{info().summary}",
                    oninput: move |evt| handle_change("summary", evt.value())
                }
            }
        }
    }
} 