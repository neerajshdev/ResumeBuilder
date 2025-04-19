use std::rc::Rc;

use dioxus::prelude::*;

use crate::{domain::Resume, presentation::ResumeViewModel};

#[component]
pub fn ResumePreview(resume: Resume, selected_theme: usize, themes: Vec<(&'static str, &'static str)>) -> Element {

    let (_, theme_bg) = themes[selected_theme];

    rsx! {
        div { class: "border p-6 bg-white shadow rounded {theme_bg}",
            div { class: "mb-6 border-b pb-4",
                h1 { class: "text-2xl font-bold", "{resume.personal_info.name}" }
                div { class: "flex flex-wrap gap-2 text-sm text-gray-600",
                    if !resume.personal_info.email.is_empty() {
                        span { "{resume.personal_info.email}" }
                    }
                    if !resume.personal_info.phone.is_empty() {
                        span { "{resume.personal_info.phone}" }
                    }
                    if !resume.personal_info.location.is_empty() {
                        span { "{resume.personal_info.location}" }
                    }
                }
            }

            if !resume.personal_info.summary.is_empty() {
                div { class: "mt-4",
                    h2 { class: "text-lg font-bold border-b", "Summary" }
                    p { class: "mt-2", "{resume.personal_info.summary}" }
                }
            }

            // You can continue with rendering Education, Experience, etc.
        }
    }
}
