use dioxus::prelude::*;

use crate::footer;
use crate::shiny_dex::dex_by_type::DexByType;

static DEFAULT_DEX: &str = "paldea";
static DEFAULT_TYPE: &str = "normal";

pub static DEXES: [&str; 3] = ["paldea", "kitakami", "blueberry"];

pub static TYPES: [&str; 18] = [
    "normal", "grass", "fire", "water", "electric", "ice", "fighting", "poison", "ground",
    "flying", "psychic", "bug", "rock", "ghost", "dragon", "dark", "steel", "fairy",
];

#[component]
pub fn ShinyDex() -> Element {
    let mut dex = use_signal(|| DEFAULT_DEX.to_string());
    let mut pokemon_type = use_signal(|| DEFAULT_TYPE.to_string());

    rsx! {
        div { display: "flex", flex_direction: "row", justify_content: "space-between",
            h1 { "Welcome to the shiny hunters pokédex" }
            h1 { a { href: "/shiny/favourites", "View your favourites" } }
        }
        div { display: "flex", flex_direction: "row",
            select {
                class: "bg-white font-bold py-2 px-4 rounded",
                width: "20%",
                oninput: move |event| {
                    dex.set(event.data.value().clone());
                },
                for dex in DEXES.iter() {
                    option { value: *dex, "{dex}" }
                }
            }
            select {
                class: "bg-white font-bold py-2 px-4 rounded",
                width: "20%",
                oninput: move |event| {
                    pokemon_type.set(event.data.value().clone());
                },
                for key in TYPES.iter() {
                    option { value: *key, "{key}" }
                }
            }
        }
        DexByType { dex: dex, pokemon_type: pokemon_type }
        footer::Footer {}
    }
}
