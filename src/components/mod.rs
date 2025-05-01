//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

use dioxus::prelude::*;
mod dialog;

pub use dialog::Dialog;

#[derive(Clone, PartialEq)]
pub enum LocalElement {
    String(String),
    Element(Element),
}

impl From<Element> for LocalElement {
    fn from(element: Element) -> Self {
        LocalElement::Element(element)
    }
}

impl From<String> for LocalElement {
    fn from(string: String) -> Self {
        LocalElement::String(string)
    }
}