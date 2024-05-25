use dioxus::{html::section, prelude::*};
use dioxus_free_icons::icons::fa_brands_icons::{FaGithub, FaLinkedin, FaTwitter};
use dioxus_free_icons::Icon;

#[component]
pub fn hero_section() -> Element {

    rsx! {
        section {
            class: "bg-emerald-100 py-24",
            div {
                class: "align-element grid md:grid-cols-2 items-center gap-8",
                article {
                    h1 {
                        class: "text-7xl font-bold tracking-wider",
                        "I'm Eydi"
                    }
                    p {
                        class: "mt-4 text-3xl text-slate-700 capitalize tracking-wide",
                        "Front-end developer"
                    }
                    p {
                        class: "mt-2 text-lg text-slate-700 capitalize tracking-wide",
                        "turning ideas into interactive reality"
                    }
                    div {
                        class: "flex gap-x-4 mt-4",
                        a {
                            href: "#",
                            Icon {
                                class: "h-8 w-8 text-slate-500 hover:text-black duration-300",
                                icon: FaGithub
                            }
                        }
                        a {
                            href: "#",
                            Icon {
                                class: "h-8 w-8 text-slate-500 hover:text-black duration-300",
                                icon: FaLinkedin
                            }
                        }
                        a {
                            href: "#",
                            Icon {
                                class: "h-8 w-8 text-slate-500 hover:text-black duration-300",
                                icon: FaTwitter
                            }
                        }
                    }
                }

                article {
                    class: "hidden md:block",
                    img {
                        src: "hero.svg",
                        class: "h-80 lg:h-96"
                    }
                }
            }
        }
    }
}