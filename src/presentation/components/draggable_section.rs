use dioxus::prelude::*;

#[component]
pub fn DraggableSection(
    index: usize,
    total_sections: usize,
    on_move_up: Option<EventHandler<()>>,
    on_move_down: Option<EventHandler<()>>,
    children: Element
) -> Element {
    rsx! {
        div {
            class: "border rounded bg-white shadow-sm mb-4 relative",
            
            // Drag handle
            div {
                class: "absolute top-0 right-0 bg-gray-100 p-2 rounded-bl cursor-move",
                // Drag icon
                svg {
                    class: "w-5 h-5 text-gray-500",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M4 6h16M4 12h16M4 18h16"
                    }
                },
                // This would connect to a drag-and-drop library in a real implementation
                // For now, we'll use buttons to simulate moving up/down as a simple approximation
                div {
                    class: "flex mt-1 gap-1",
                    if index > 0 && on_move_up.is_some() {
                        button {
                            class: "p-1 bg-gray-200 rounded hover:bg-gray-300",
                            onclick: move |_| {
                                if let Some(handler) = &on_move_up {
                                    handler.call(());
                                }
                            },
                            "↑"
                        }
                    }
                    if index < total_sections - 1 && on_move_down.is_some() {
                        button {
                            class: "p-1 bg-gray-200 rounded hover:bg-gray-300",
                            onclick: move |_| {
                                if let Some(handler) = &on_move_down {
                                    handler.call(());
                                }
                            },
                            "↓"
                        }
                    }
                }
            },
            
            div {
                class: "p-4",
                {children}
            }
        }
    }
} 