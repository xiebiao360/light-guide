use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    show: bool,
    on_close: EventHandler<()>,
    children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    if !props.show {
        rsx! {}
    } else {
        rsx! {
            div {
                class: "dialog-overlay",
                onclick: move |_| props.on_close.call(()),

                div {
                    class: "dialog-content",
                    onclick: |e| e.stop_propagation(),

                    {props.children}
                }
            }
        }
    }
}
