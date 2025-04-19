use dioxus::prelude::*;
use crate::domain::PersonalInfo;

#[component]
pub fn PersonalInfoForm(personal_info: PersonalInfo, on_change: EventHandler<PersonalInfo>) -> Element {
    // Create a signal to hold the personal info state
    let mut info_signal = use_signal(|| personal_info.clone());
    
    // Function to update a specific field
    let mut update_field = move |field: &str, value: String| {
        info_signal.with_mut(|info| {
            // Update the appropriate field
            match field {
                "name" => info.name = value,
                "email" => info.email = value,
                "phone" => info.phone = value,
                "website" => info.website = value,
                "linkedin" => info.linkedin = value,
                "github" => info.github = value,
                "location" => info.location = value,
                "summary" => info.summary = value,
                _ => {}
            }
        });
        
        // Notify parent component of change
        on_change.call(info_signal.read().clone());
    };
    
    // Get the current value from the signal for display
    let info = info_signal.read();
    
    rsx! {
        form { class: "space-y-6",
            h2 { class: "text-xl font-bold mb-4", "Personal Information" }
            
            // Name field
            div { class: "space-y-2",
                label { class: "block text-sm font-medium text-gray-700", for: "name", "Full Name*" }
                input {
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                    id: "name",
                    r#type: "text",
                    value: "{info.name}",
                    placeholder: "John Doe",
                    required: true,
                    oninput: move |evt| update_field("name", evt.value()),
                }
            }
            
            // Two-column layout for contact information
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                // Email field
                div { class: "space-y-2",
                    label { class: "block text-sm font-medium text-gray-700", for: "email", "Email*" }
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                        id: "email",
                        r#type: "email",
                        value: "{info.email}",
                        placeholder: "john.doe@example.com",
                        required: true,
                        oninput: move |evt| update_field("email", evt.value()),
                    }
                }
                
                // Phone field
                div { class: "space-y-2",
                    label { class: "block text-sm font-medium text-gray-700", for: "phone", "Phone" }
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                        id: "phone",
                        r#type: "tel",
                        value: "{info.phone}",
                        placeholder: "(555) 123-4567",
                        oninput: move |evt| update_field("phone", evt.value()),
                    }
                }
                
                // Location field
                div { class: "space-y-2",
                    label { class: "block text-sm font-medium text-gray-700", for: "location", "Location" }
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                        id: "location",
                        r#type: "text",
                        value: "{info.location}",
                        placeholder: "City, State",
                        oninput: move |evt| update_field("location", evt.value()),
                    }
                }
                
                // Website field
                div { class: "space-y-2",
                    label { class: "block text-sm font-medium text-gray-700", for: "website", "Website" }
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                        id: "website",
                        r#type: "url",
                        value: "{info.website}",
                        placeholder: "https://example.com",
                        oninput: move |evt| update_field("website", evt.value()),
                    }
                }
            }
            
            // Social media profile links
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                // LinkedIn field
                div { class: "space-y-2",
                    label { class: "block text-sm font-medium text-gray-700", for: "linkedin", 
                        span { "LinkedIn " }
                        span { class: "text-gray-500 text-xs", "(optional)" }
                    }
                    div { class: "flex",
                        span { class: "inline-flex items-center px-3 border border-r-0 border-gray-300 bg-gray-50 text-gray-500 rounded-l-md", "linkedin.com/in/" }
                        input {
                            class: "w-full px-3 py-2 border border-gray-300 rounded-r-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                            id: "linkedin",
                            r#type: "text",
                            value: "{info.linkedin}",
                            placeholder: "johndoe",
                            oninput: move |evt| update_field("linkedin", evt.value()),
                        }
                    }
                }
                
                // GitHub field
                div { class: "space-y-2",
                    label { class: "block text-sm font-medium text-gray-700", for: "github", 
                        span { "GitHub " }
                        span { class: "text-gray-500 text-xs", "(optional)" }
                    }
                    div { class: "flex",
                        span { class: "inline-flex items-center px-3 border border-r-0 border-gray-300 bg-gray-50 text-gray-500 rounded-l-md", "github.com/" }
                        input {
                            class: "w-full px-3 py-2 border border-gray-300 rounded-r-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                            id: "github",
                            r#type: "text",
                            value: "{info.github}",
                            placeholder: "johndoe",
                            oninput: move |evt| update_field("github", evt.value()),
                        }
                    }
                }
            }
            
            // Summary field
            div { class: "space-y-2",
                label { class: "block text-sm font-medium text-gray-700", for: "summary", "Professional Summary" }
                textarea {
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                    id: "summary",
                    rows: "4",
                    value: "{info.summary}",
                    placeholder: "A brief summary highlighting your professional background and key strengths...",
                    oninput: move |evt| update_field("summary", evt.value()),
                }
                p { class: "text-sm text-gray-500 mt-1", "Write a concise summary that highlights your most relevant skills and experience." }
            }
            
            // Tips section
            div { class: "mt-6 p-4 bg-blue-50 rounded-md border border-blue-200",
                h3 { class: "text-sm font-medium text-blue-800", "Tips for a Great Profile" }
                ul { class: "mt-2 text-sm text-blue-700 list-disc list-inside space-y-1",
                    li { "Keep your summary concise and focused on your key strengths" }
                    li { "Use a professional email address" }
                    li { "Include social media profiles only if they're professional" }
                    li { "Location can be as simple as 'City, State' or 'Remote'" }
                }
            }
        }
    }
} 