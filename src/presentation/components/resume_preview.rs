use dioxus::prelude::*;
use crate::domain::models::Resume;

#[component]
pub fn ResumePreview(
    resume: Resume,
    theme_bg: &'static str
) -> Element {
    rsx! {
        div {
            class: format!("border p-6 bg-white shadow rounded {}", theme_bg),
            
            // Personal info
            div {
                class: "mb-6 border-b pb-4",
                h1 {
                    class: "text-2xl font-bold",
                    "{resume.personal_info.name}"
                },
                div {
                    class: "flex flex-wrap gap-2 text-sm text-gray-600",
                    if !resume.personal_info.email.is_empty() {
                        span {
                            "{resume.personal_info.email}"
                        }
                    },
                    if !resume.personal_info.phone.is_empty() {
                        span {
                            " | {resume.personal_info.phone}"
                        }
                    },
                    if !resume.personal_info.location.is_empty() {
                        span {
                            " | {resume.personal_info.location}"
                        }
                    }
                }
            },
            
            // Summary
            if !resume.personal_info.summary.is_empty() {
                div {
                    class: "mt-4",
                    h2 {
                        class: "text-lg font-bold border-b",
                        "Summary"
                    },
                    p {
                        class: "mt-2",
                        "{resume.personal_info.summary}"
                    }
                }
            },
            
            // Education
            if !resume.education.is_empty() {
                div {
                    class: "mt-6",
                    h2 {
                        class: "text-lg font-bold border-b mb-2",
                        "Education"
                    },
                    for edu in resume.education.iter() {
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
                            if !edu.location.is_empty() {
                                div {
                                    class: "text-sm text-gray-600",
                                    "{edu.location}"
                                }
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
            if !resume.experience.is_empty() {
                div {
                    class: "mt-6",
                    h2 {
                        class: "text-lg font-bold border-b mb-2",
                        "Experience"
                    },
                    for exp in resume.experience.iter() {
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
                            if !exp.location.is_empty() {
                                div {
                                    class: "text-sm text-gray-600",
                                    "{exp.location}"
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
                            }
                        }
                    }
                }
            }
        }
    }
}
