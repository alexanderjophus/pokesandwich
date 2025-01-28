use dioxus::prelude::*;

pub fn Home() -> Element {
    rsx! {
        div {
            h1 { "Home" }
            p { "Click on a navbar menu to get started." }
        }
    }
}
