use dioxus::prelude::*;

use super::LocalElement;

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    /// The title of the dialog.
    title: Option<LocalElement>,
    /// The content of the dialog.
    children: Element,
    /// Whether the dialog is visible or not.
    show: bool,
}

#[allow(non_snake_case)]
pub fn Dialog(props: DialogProps) -> Element {
    let title_element = match &props.title {
        Some(LocalElement::String(s)) => rsx!(div { "{s}" }),
        Some(LocalElement::Element(e)) => e.clone(),
        None => rsx!(),
    };
    rsx!(
        div {
            class: "dialog-overlay",
            display: if props.show { "block" } else { "none" },
            div {
                class: "dialog-content",

                div {
                    class: "dialog-header",
                    // style: "padding: 16px; border-bottom: 1px solid #e0e0e0;",
                    {title_element}
                }
                div {
                    class: "dialog-body",
                    // style: "padding: 16px;",
                    {props.children}
                }
                div {
                    class: "dialog-footer",
                    // style: "padding: 16px; border-top: 1px solid #e0e0e0;",
                    button {
                        onclick: |_| {
                            // Close the dialog
                        },
                        "Close"
                    }
                }
            }
        }
    )
}
