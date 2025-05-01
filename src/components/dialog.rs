use dioxus::{html::g::r, prelude::*};

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

pub fn Dialog(props: DialogProps) -> Element {
    let title_element = match &props.title {
        Some(LocalElement::String(s)) => rsx!(div { "{s}" }),
        Some(LocalElement::Element(e)) => e.clone(),
        None => rsx!(),
    };
    rsx!(
        div {
            display: if props.show { "block" } else { "none" },
            position: "fixed",
            top: 0,
            left: 0,
            width: "100vw",
            height: "100vh",
            background: "rgba(0, 0, 0, 0.4)",
            div {
                position: "absolute",
                top: "50%",
                left: "50%",
                transform: "translate(-50%, -50%)",
                border_radius: "8px",
                background: "#fff",

                div {
                    class: "dialog-header",
                    style: "padding: 16px; border-bottom: 1px solid #e0e0e0;",
                    {title_element}
                }
                div {
                    class: "dialog-body",
                    style: "padding: 16px;",
                    {props.children}
                }
                div {
                    class: "dialog-footer",
                    style: "padding: 16px; border-top: 1px solid #e0e0e0;",
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
