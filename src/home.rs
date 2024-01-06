use dioxus::prelude::*;

use crate::consts::{DEXES, TYPES_INGREDIENTS};
use crate::dex_by_type;
use crate::footer;

static DEFAULT_DEX: &str = "paldea";
static DEFAULT_TYPE: &str = "normal";

#[derive(Clone, Debug)]
pub enum SearchState {
    Unset,
    Set { dex: String, pokemon_type: String },
}

#[inline_props]
pub fn Home(cx: Scope) -> Element {
    use_shared_state_provider(cx, || SearchState::Unset);
    let search_state = use_shared_state::<SearchState>(cx).unwrap();

    let dex = use_state(cx, || DEFAULT_DEX.to_string());
    let pokemon_type = use_state(cx, || DEFAULT_TYPE.to_string());

    cx.render(rsx! {
        h1 { "Welcome to the shiny hunters pokédex" }
        div { display: "flex", flex_direction: "row",
            select {
                width: "20%",
                oninput: move |event| {
                    dex.set(event.data.value.clone());
                },
                for dex in DEXES.iter() {
                    option {
                        value: *dex, "{dex}"
                    }
                }
            }
            select {
                width: "20%",
                oninput: move |event| {
                    pokemon_type.set(event.data.value.clone());
                },
                for key in TYPES_INGREDIENTS.keys() {
                    option {
                        value: *key, "{key}"
                    }
                }
            }
            button {
                onclick: move |_| {
                    *search_state.write() = SearchState::Set {
                        dex: dex.to_string(),
                        pokemon_type: pokemon_type.to_string(),
                    };
                },
                "Refresh"
            }
        }
        if let SearchState::Set { dex, pokemon_type } = search_state.read().clone() {
            render!(
                dex_by_type::DexByType { dex: dex, pokemon_type: pokemon_type }
            )
        }
        footer::Footer {}
    })
}
