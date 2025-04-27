use crate::Route;
use dioxus::{html::div, prelude::*};

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            Left {}
            Content {}
        }
    }
}

#[component]
pub fn Left() -> Element {
    rsx! {
        div {
            id: "left",
            div {
                id: "env",
                input {  }
            }
        }
    }
}

#[component]
pub fn Content() -> Element {
    rsx! {
        div {
            id: "content",
            Header {}
            MainContent {}
        }
    }
}

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            id: "header",
            "header",
        }
    }
}

#[component]
pub fn MainContent() -> Element {
    rsx! {
        div {
            id: "main-content",
            // The Outlet component will render the current route's component
            Outlet::<Route> {}
        }
    }
}
