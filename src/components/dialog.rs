use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    show: bool,
    on_close: EventHandler<()>,
    children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let overlay_class = use_signal(|| String::from("dialog-overlay"));
    let content_class = use_signal(|| String::from("dialog-content"));

    use_effect(move || {
        to_owned![overlay_class, content_class];
        if props.show {
            overlay_class.set("dialog-overlay show".to_string());
            content_class.set("dialog-content show".to_string());
        } else {
            overlay_class.set("dialog-overlay".to_string());
            content_class.set("dialog-content".to_string());
        }
    });

    if !props.show {
        rsx! {}
    } else {
        rsx! {
            div {
                class: overlay_class.read().to_string(),
                onclick: move |_| props.on_close.call(()),

                div {
                    class: content_class.read().to_string(),
                    onclick: |e| e.stop_propagation(),

                    {props.children}
                }
            }
        }
    }
}
