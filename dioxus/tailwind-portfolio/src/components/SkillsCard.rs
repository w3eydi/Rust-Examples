use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_brands_icons::{FaReact, FaHtml5, FaJs};
use dioxus_free_icons::Icon;


#[component]
pub fn skills_card(icon_type: String, title: String, text: String) -> Element {

    rsx! {
        article {
            span {
                class: "h-16 w-16",
                Icon {
                    class: "h-16 w-16 text-emerald-500",
                    icon: FaHtml5
                }
            }
            h4 {
                class: "mt-6 font-bold",
                {title}
            }
            p {
                class: "mt-2 text-slate-500",
                {text}
            }
        }
    }
}