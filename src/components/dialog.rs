use dioxus::{logger::tracing::info, prelude::*};

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    show: bool,
    on_close: EventHandler<()>,
    children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let mut overlay_class = use_signal(|| "dialog-overlay");
    let mut content_class = use_signal(|| "dialog-content");

    use_effect(move || {
        info!("Dialog show: {}", props.show);
        if props.show {
            overlay_class.set("dialog-overlay show");
            content_class.set("dialog-content show");
        } else {
            overlay_class.set("dialog-overlay");
            content_class.set("dialog-content");
        }
    });

    let overlay_class_str: &str = &overlay_class.read();
    let content_class_str: &str = &content_class.read();

    if !props.show {
        rsx! {}
    } else {
        rsx! {
            div {
                class: overlay_class_str,
                onclick: move |_| props.on_close.call(()),

                div {
                    class: content_class_str,
                    onclick: |e| e.stop_propagation(),

                    {props.children}
                }
            }
        }
    }
}
