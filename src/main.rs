#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub static BASE_API_URL: &str = "https://pokeapi.co/api/v2/";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pokedex {
    pokemon_entries: Vec<PokemonEntry>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PokemonEntry {
    entry_number: i64,
    pokemon_species: PokemonSpecies,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PokemonSpecies {
    name: String,
    url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PokemonType {
    pokemon: Vec<Pokemon>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pokemon {
    pokemon: PokemonReference,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PokemonReference {
    name: String,
}

fn main() {
    dioxus_web::launch(App);
}

pub fn App(cx: Scope) -> Element {
    let dex = use_state(cx, || "paldea".to_string());
    let pokemon_type = use_state(cx, || "normal".to_string());

    cx.render(rsx! {
        input {
            value: "{dex}",
            oninput: move |e| {
                dex.set(e.data.value.clone());
            }
        }
        input {
            value: "{pokemon_type}",
            oninput: move |e| {
                pokemon_type.set(e.data.value.clone());
            }
        }
        div {
            display: "flex",
            flex_direction: "row",
            width: "100%",
            Search {
                dex: dex.clone(),
                pokemon_type: pokemon_type.clone(),
            }
        }
    })
}

#[derive(PartialEq, Props, Clone)]
struct SearchProps {
    dex: UseState<String>,
    pokemon_type: UseState<String>,
}

fn Search(cx: Scope<SearchProps>) -> Element {
    let dexes = use_future(cx, &cx.props.clone(), |_| {
        get_pokedex(cx.props.dex.to_string())
    });
    let pokemon_types = use_future(cx, &cx.props.clone(), |_| {
        get_types(cx.props.pokemon_type.to_string())
    });

    match (dexes.value(), pokemon_types.value()) {
        (Some(Ok(dex)), Some(Ok(types))) => render_dex(cx, dex.clone(), types.clone()),
        (Some(Err(err)), _) => render! {"An error occurred while loading dexes {err}"},
        (_, Some(Err(err))) => render! {"An error occurred while loading dexes {err}"},
        _ => render! {"Loading items"},
    }
}

fn render_dex(cx: Scope<SearchProps>, dex: Pokedex, types: PokemonType) -> Element {
    let pokemon_names: Vec<String> = types
        .pokemon
        .iter()
        .map(|pokemon| pokemon.pokemon.name.clone())
        .collect();

    let pokemon_entries: Vec<PokemonEntry> = dex
        .pokemon_entries
        .into_iter()
        .filter(|entry| pokemon_names.contains(&entry.pokemon_species.name))
        .collect();

    let pokedex_entries: Vec<DexItem> = pokemon_entries
        .into_iter()
        .map(|entry| DexItem {
            id: entry.entry_number,
            name: entry.pokemon_species.name,
            url: entry.pokemon_species.url,
        })
        .collect();

    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            width: "100%",
            pokedex_entries.into_iter().map(|entry| rsx! {
                DexListing {dex: entry }
            })
        }
    })
}

#[inline_props]
fn DexListing(cx: Scope, dex: DexItem) -> Element {
    let DexItem { id, name, url, .. } = dex;

    cx.render(rsx! {
        div {
            padding: "0.5rem",
            position: "relative",
            div {
                display: "flex",
                flex_direction: "row",
                div {
                    font_weight: "bold",
                    padding_right: "0.5rem",
                    "{id}"
                }
                div {
                    padding_right: "0.5rem",
                    "{name}"
                }
                div {
                    padding_right: "0.5rem",
                    "{url}"
                }
            }
        }
    })
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryPageData {
    #[serde(flatten)]
    pub item: PokemonReference,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DexItem {
    pub id: i64,
    pub name: String,
    pub url: String,
}

pub async fn get_pokedex(dex: String) -> Result<Pokedex, reqwest::Error> {
    let url = format!("{}pokedex/{}", BASE_API_URL, dex);
    reqwest::get(&url).await?.json().await
}

pub async fn get_types(poketype: String) -> Result<PokemonType, reqwest::Error> {
    let url = format!("{}type/{}", BASE_API_URL, poketype);
    reqwest::get(&url).await?.json().await
}
