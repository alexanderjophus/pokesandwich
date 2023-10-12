use dioxus::prelude::*;

use crate::consts::{DEXES, TYPES_INGREDIENTS};
use crate::footer;

#[inline_props]
pub fn Home(cx: Scope) -> Element {
    let dex = use_state(cx, || "".to_string());
    let pokemon_type = use_state(cx, || "".to_string());

    cx.render(rsx! {
        h1 { "Welcome to the pokemon dex" }
        p { "Select a dex and a type to get started" }
        div {
            display: "flex",
            flex_direction: "row",
            select {
                width: "20%",
                oninput: move |event| {
                    dex.set(event.data.value.clone());
                },
                option {
                    value: "",
                    "Select a dex"
                }
                for dex in DEXES.iter() {
                    option {
                        value: *dex,
                        "{dex}"
                    }
                }
            }
            select {
                width: "20%",
                oninput: move |event| {
                    pokemon_type.set(event.data.value.clone());
                },
                option {
                    value: "",
                    "Select a type"
                }
                for key in TYPES_INGREDIENTS.keys() {
                    option {
                        value: *key,
                        "{key}"
                    }
                }
            }
            a {
                href: "/dex/{dex.get()}/type/{pokemon_type.get()}",
                "Search"
            }
        }
        footer::Footer {}
    })
}
