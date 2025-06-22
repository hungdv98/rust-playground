#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    launch(App);
}

fn App() -> Element {
    rsx!(
        div { 
            h1 { "Hello World!" }
        }
    )
}
