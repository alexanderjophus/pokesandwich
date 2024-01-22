use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            h1 { "Home" }
            p { "Click on a navbar menu to get started." }
        }
    })
}
