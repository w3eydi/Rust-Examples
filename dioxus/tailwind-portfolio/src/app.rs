#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::navbar::main_nav;


#[component]
pub fn App() -> Element {

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        main_nav {}
    }
}