use dioxus::prelude::*;

// Tab enum for navigation
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Personal,
    Education,
    Experience,
    Skills,
    Projects,
    Preview,
}

impl Tab {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Personal => "Personal Info",
            Self::Education => "Education",
            Self::Experience => "Experience",
            Self::Skills => "Skills",
            Self::Projects => "Projects",
            Self::Preview => "Preview",
        }
    }
    
    pub fn all() -> Vec<Self> {
        vec![
            Self::Personal,
            Self::Education,
            Self::Experience,
            Self::Skills,
            Self::Projects,
            Self::Preview,
        ]
    }
}

// TabBar component for navigation
#[component]
pub fn TabBar(
    current_tab: Tab,
    on_tab_change: EventHandler<Tab>,
) -> Element {
    rsx! {
        div { class: "flex mb-4 space-x-2 justify-center",
            for tab in Tab::all() {
                button { 
                    class: if current_tab == tab { 
                        "px-4 py-2 bg-blue-500 text-white rounded" 
                    } else { 
                        "px-4 py-2 bg-gray-200 rounded" 
                    },
                    onclick: move |_| on_tab_change.call(tab),
                    "{tab.name()}" 
                }
            }
        }
    }
} 