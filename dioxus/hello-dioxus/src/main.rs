use dioxus::prelude::*;
use tracing::Level;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        book_list {}
    }
}

#[component]
fn book_list() -> Element {
    rsx!{
        section {
            class: "booklist",
            book_definition {}
            book_definition {}
            book_definition {}
            book_definition {}
            book_definition {}
        }
    }
}

#[component]
fn book_definition() -> Element {
    rsx!{
        article {
            class: "book",
            book_title {}
            book_author {}
            book_placeholder {}
        }
    }
}

#[component]
fn book_title() -> Element {
    rsx!{
        h2 {"Atomic Habits"}
    }
}

#[component]
fn book_author() -> Element {
    rsx!{
        h4 { "James Clear" }
    }
}

#[component]
fn book_placeholder() -> Element {
    rsx! {
        img {
            src: "https://m.media-amazon.com/images/I/81YkqyaFVEL._SY466_.jpg",
            alt: "Interesting Facts For Curious Minds"
        }
    }
}
