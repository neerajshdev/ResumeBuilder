use dioxus::prelude::*;

pub type Theme = (&'static str, &'static str);

#[component]
pub fn ThemeSelector(
    themes: Vec<Theme>,
    selected_theme: usize,
    on_theme_select: EventHandler<usize>
) -> Element {
    rsx! {
        div {
            class: "mb-4 p-4 border rounded bg-white shadow-sm",
            h3 {
                class: "text-lg font-semibold mb-2",
                "Choose Theme"
            },
            div {
                class: "grid grid-cols-3 gap-2",
                for (index, (theme_name, theme_bg)) in themes.iter().enumerate() {
                    div {
                        class: "relative cursor-pointer",
                        onclick: move |_| on_theme_select.call(index),
                        div {
                            class: format!("border-2 {} p-2 rounded transition-colors {}", 
                                if selected_theme == index { "border-blue-500" } else { "border-gray-200" }, 
                                theme_bg
                            ),
                            div {
                                class: "h-10 flex items-center justify-center",
                                "{theme_name}"
                            }
                        }
                    }
                }
            }
        }
    }
} 