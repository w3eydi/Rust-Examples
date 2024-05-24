use dioxus::prelude::*;

use crate::data_structure::links_generator;

#[component]
pub fn main_nav() -> Element {
    let links = links_generator(4);

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
                    for link in links{
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