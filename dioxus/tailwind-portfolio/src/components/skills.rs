use dioxus::prelude::*;

use crate::{components::{props::SectionTitle, section_title::section_title, SkillsCard::skills_card}, data_structure::SkillWrapper};


#[component]
pub fn Skills() -> Element {
    let SKILLS_JSON_DATA: &str = include_str!("../../src/data.json");
    let skills_wrapper: SkillWrapper = serde_json::from_str(SKILLS_JSON_DATA).unwrap();
    rsx! {
        section {
            class: "py-20 align-element",
            id: "skills",
            section_title { title: SectionTitle {text: "tech stack".to_string()} },
            div {
                class: "py-16 grid md:grid-cols-2 lg:grid-cols-3 gap-8",
                for skill in skills_wrapper.skills {
                    skills_card {
                        key: "{skill.id}",
                        icon_type: "{skill.icon}",
                        title: "{skill.title}",
                        text: "{skill.text}"
                    }
                }
            }
        }
    }
}