use dioxus::prelude::*;
use crate::components::props::SectionTitle;

#[component]
pub fn section_title(title: SectionTitle) -> Element {

    rsx! {
        div {
            class: "border-b border-gray-200 pb-5",
            h2 {
                class: "text-3xl font-medium tracking-wider capitalize",
                {title.text}
            }
        }
    }
}