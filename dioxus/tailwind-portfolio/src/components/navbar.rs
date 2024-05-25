
use dioxus::prelude::*;

use crate::data_structure::LinksWrapper;

#[component]
pub fn main_nav() -> Element {
    let LINK_JSON_DATA: &str = include_str!("../../src/data.json");
    let links_wrapper: LinksWrapper = serde_json::from_str(LINK_JSON_DATA).unwrap();
    
    rsx!{
        nav {
            class: "bg-emerald-100",
            div {
                class: "mx-auto max-w-7xl px-8 py-4 flex flex-col sm:flex-row
                sm:gap-x-16 sm:items-center sm:py-8",
                h2 {
                    class: "text-3xl font-bold",
                    "Web" span { class: "text-emerald-600", " Dev"}
                }
                div {
                    class: "flex gap-x-3",
                    for link in links_wrapper.links {
                        a {
                            class: "capitalize text-lg tracking-wide hover:text-emerald-600 duration-300",
                            key: "{link.id}",
                            href: "{link.href}",
                            "{link.text}"
                        }
                    }
                }
            }
        }
    }
}