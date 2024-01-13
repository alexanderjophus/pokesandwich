use dioxus::prelude::*;

use crate::consts::{DEXES, TYPES};
use crate::dex_by_type;
use crate::footer;

static DEFAULT_DEX: &str = "paldea";
static DEFAULT_TYPE: &str = "normal";

#[component]
pub fn Home(cx: Scope) -> Element {
    let dex = use_state(cx, || DEFAULT_DEX.to_string());
    let pokemon_type = use_state(cx, || DEFAULT_TYPE.to_string());

    cx.render(rsx! {
        h1 { "Welcome to the shiny hunters pokédex" }
        div { display: "flex", flex_direction: "row",
            select {
                class: "bg-white font-bold py-2 px-4 rounded",
                width: "20%",
                oninput: move |event| {
                    dex.set(event.data.value.clone());
                },
                for dex in DEXES.iter() {
                    option { value: *dex, "{dex}" }
                }
            }
            select {
                class: "bg-white font-bold py-2 px-4 rounded",
                width: "20%",
                oninput: move |event| {
                    pokemon_type.set(event.data.value.clone());
                },
                for key in TYPES.iter() {
                    option { value: *key, "{key}" }
                }
            }
        }
        render!(
            dex_by_type::DexByType { dex: dex.to_string(), pokemon_type: pokemon_type.to_string() }
        ),
        footer::Footer {}
    })
}
