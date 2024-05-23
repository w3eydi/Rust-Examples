use dioxus::prelude::*;

#[component]
pub fn test_eleme() -> Element {
    rsx!{
        nav {
            div {
                class: "bg-gray-200 text-white",
                "test deneme 123"
            }
        }
    }
}