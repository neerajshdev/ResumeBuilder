use dioxus::prelude::*;

#[component]
pub fn ExportModal(
    show: bool,
    theme_name: String,
    on_close: EventHandler<()>,
    on_download: EventHandler<()>
) -> Element {
    if !show {
        return rsx!{};
    }

    rsx! {
        div {
            class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            div {
                class: "bg-white rounded-lg p-6 w-96 shadow-xl",
                h3 {
                    class: "text-xl font-bold mb-4",
                    "Export Resume"
                },
                p {
                    class: "mb-4",
                    "Your resume has been prepared for export with the '{theme_name}' theme."
                },
                div {
                    class: "flex justify-between",
                    button {
                        class: "px-4 py-2 bg-gray-300 rounded hover:bg-gray-400 transition-colors",
                        onclick: move |_| on_close.call(()),
                        "Close"
                    },
                    button {
                        class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors",
                        onclick: move |_| {
                            on_download.call(());
                        },
                        "Download PDF"
                    }
                }
            }
        }
    }
} 