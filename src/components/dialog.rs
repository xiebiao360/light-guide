use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    /// The title of the dialog.
    title: Option<String>,
    /// The header of the dialog. This will override the title if both are provided.
    header: Option<Element>,
    /// The function to call when the dialog is closed.
    onclose: EventHandler<()>,
    /// The function to call when the dialog is confirmed.
    onconfirm: EventHandler<()>,
    /// The function to call when the dialog is cancelled.
    oncancel: Option<EventHandler<()>>,
    /// The function to call when the dialog is validated.
    validate: Option<Callback<(), Option<Vec<String>>>>,
    /// The content of the dialog.
    children: Element,
    /// Whether the dialog is visible or not.
    show: bool,
}

#[allow(non_snake_case)]
pub fn Dialog(props: DialogProps) -> Element {
    let mut errors = use_signal(|| Vec::new());
    let header_element = match (props.title, props.header) {
        (Some(title), _) => rsx! {
            div { "{title}" }
        },
        (_, Some(header)) => header,
        _ => rsx!(),
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
                    {header_element}
                }

                div {
                    width: "100%",
                    height: "1px",
                    background: "#e0e0e0",
                    margin: "10px 0",
                }

                div {
                    class: "dialog-body",
                    {props.children}
                }
                if !errors.is_empty() {
                    div {
                        border: "1px solid red",
                        border_radius: "4px",
                        color: "red",
                        margin: "10px 0",
                        padding: "5px 10px",
                        for error in errors.read().iter() {
                            div { "{error}" }
                        }
                    }
                }
                div {
                    class: "dialog-footer",
                    button {
                        class: "btn primary",
                        onclick: move |_| {
                            if let Some(validate) = &props.validate {
                                // Call the validate function and handle the result
                                let result = validate.call(());
                                if let Some(error) = result {
                                    // Handle validation error
                                    // You can show an error message or take any other action
                                    errors.set(error);
                                    return;
                                }
                            }
                            // Handle confirm action
                            props.onconfirm.call(());
                            // Close the dialog
                            props.onclose.call(());
                        },
                        "确定"
                    }
                    button {
                        class: "btn",
                        onclick: move |_| {
                            // Handle cancel action
                            if let Some(oncancel) = &props.oncancel {
                                oncancel.call(());
                            }
                            // Close the dialog
                            props.onclose.call(());
                        },
                        "取消"
                    }
                }
            }
        }
    )
}
