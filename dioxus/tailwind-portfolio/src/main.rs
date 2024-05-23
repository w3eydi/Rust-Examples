#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

mod app;
mod components;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(app::App);
}


