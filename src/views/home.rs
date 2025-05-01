use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
#[allow(non_snake_case)]
pub fn Home() -> Element {
    rsx! {
        div {
            id: "home",
            "Welcome to the Dioxus App!",
        }
    }
}
