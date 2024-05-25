#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::navbar::main_nav;
use crate::components::hero::hero_section;

#[component]
pub fn App() -> Element {

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        main_nav {}
        hero_section {}
    }
}