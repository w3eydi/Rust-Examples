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
            book_title {}
            book_author {}
            book_placeholder {}
        }
    }
}

#[component]
fn book_title() -> Element {
    rsx!{
        h2 {"book title"}
    }
}

#[component]
fn book_author() -> Element {
    rsx!{
        h4 { "book author" }
    }
}

#[component]
fn book_placeholder() -> Element {
    rsx! {
        h4 { "image placeholder" }
    }
}
