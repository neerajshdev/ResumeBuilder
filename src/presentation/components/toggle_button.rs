use dioxus::prelude::*;

#[component]
pub fn ToggleButton(
    is_preview_mode: bool,
    on_toggle: EventHandler<bool>
) -> Element {
    rsx! {
        div {
            class: "flex items-center bg-gray-200 rounded-full p-1",
            button {
                class: format!("px-4 py-2 rounded-full transition-colors {}",
                    if !is_preview_mode { "bg-white text-blue-800 shadow" } else { "text-gray-700" }
                ),
                onclick: move |_| on_toggle.call(false),
                "Edit"
            },
            button {
                class: format!("px-4 py-2 rounded-full transition-colors {}",
                    if is_preview_mode { "bg-white text-blue-800 shadow" } else { "text-gray-700" }
                ),
                onclick: move |_| on_toggle.call(true),
                "Preview"
            }
        }
    }
} 