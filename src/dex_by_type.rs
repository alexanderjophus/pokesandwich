use dioxus::prelude::*;
use dioxus_storage::use_persistent;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::consts::{BASE_API_URL, TYPES_INFO};

#[inline_props]
pub fn DexByType(cx: Scope, dex: String, pokemon_type: String) -> Element {
    use_shared_state_provider(cx, || FocusState::Unset);

    cx.render(rsx! {
        div { display: "flex", flex_direction: "row",
            div { overflow: "auto", max_height: "100vh", margin: "10px", width: "20%",
                Search { dex: dex.clone(), pokemon_type: pokemon_type.clone() }
            }
            div { margin: "10px", width: "80%", Focus {} }
        }
    })
}

#[derive(PartialEq, Props, Clone)]
struct SearchProps {
    dex: String,
    pokemon_type: String,
}

fn Search(cx: Scope<SearchProps>) -> Element {
    let dexes_future = use_future(cx, &cx.props.clone(), |_| {
        get_pokedex(cx.props.dex.to_string())
    });
    let pokemon_types = use_future(cx, &cx.props.clone(), |_| {
        get_types(cx.props.pokemon_type.to_string())
    });

    match (dexes_future.value(), pokemon_types.value()) {
        (Some(Ok(dex)), Some(Ok(types))) => RenderDex(cx, dex.clone(), types.clone()),
        (Some(Err(err)), _) => render! {"An error occurred while loading dexes {err}"},
        (_, Some(Err(err))) => render! {"An error occurred while loading types {err}"},
        _ => render! {"Loading items"},
    }
}

fn RenderDex(cx: Scope<SearchProps>, dex: Pokedex, types: PokemonTypeResponse) -> Element {
    let pokedex_entries = dex
        .pokemon_entries
        .iter()
        .filter(|entry| {
            types
                .pokemon
                .iter()
                .any(|pokemon| pokemon.pokemon.name == entry.pokemon_species.name)
        })
        .map(|entry| DexItem {
            id: entry.entry_number,
            name: entry.pokemon_species.name.clone(),
            url: types
                .pokemon
                .iter()
                .find(|pokemon| pokemon.pokemon.name == entry.pokemon_species.name)
                .unwrap()
                .pokemon
                .url
                .clone(),
        })
        .collect::<Vec<DexItem>>();

    cx.render(rsx! {
        div { overflow: "hidden", background_color: TYPES_INFO.get(cx.props.pokemon_type.as_str()).unwrap().color, border_radius: "50%", width: "100px", height: "100px", img { src: "/icons/{cx.props.pokemon_type.clone()}.svg" } }
        div { overflow: "auto", display: "flex", flex_direction: "column", width: "100%", DexTable { dex_entries: pokedex_entries } }
    })
}

#[inline_props]
fn DexTable(cx: Scope, dex_entries: Vec<DexItem>) -> Element {
    cx.render(rsx! {
        table { border_collapse: "collapse",
            thead {
                tr { th { "Pokemon Name" } }
            }
            tbody {
                for entry in dex_entries.iter() {
                    DexRow { dex_entry: entry.clone() }
                }
            }
        }
    })
}

#[inline_props]
fn DexRow(cx: Scope, dex_entry: DexItem) -> Element {
    let fav = use_persistent(cx, "faves", || HashSet::new());
    let focus_state = use_shared_state::<FocusState>(cx).unwrap();

    let fut = use_future(cx, &dex_entry.url, |url| {
        load_focus(focus_state.clone(), url.to_string())
    });
    cx.render(rsx! {
        tr { class: "border-2 hover:bg-gray-100 hover:ring-2 hover:ring-pink-500 hover:ring-inset",
            td {
                div { display: "flex", flex_direction: "row",
                    div {
                        width: "80%",
                        onclick: move |_| {
                            fut.restart();
                        },
                        "{dex_entry.name}"
                    }
                    div {
                        width: "20%",
                        onclick: move |_| {
                            if fav.get().contains(&dex_entry.name) {
                                fav.modify(|faves| {
                                    faves.remove(&dex_entry.name);
                                });
                            } else {
                                fav.modify(|faves| {
                                    faves.insert(dex_entry.name.clone());
                                });
                            }
                        },
                        i { class: "fa fa-heart", color: if fav.get().contains(&dex_entry.name) { "red" } else { "grey" } }
                    }
                }
            }
        }
    })
}

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
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PokemonTypeResponse {
    pokemon: Vec<Pokemon>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pokemon {
    pokemon: PokemonReference,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PokemonReference {
    url: String,
    name: String,
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

pub async fn get_types(poketype: String) -> Result<PokemonTypeResponse, reqwest::Error> {
    let url = format!("{}type/{}", BASE_API_URL, poketype);
    reqwest::get(&url).await?.json().await
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FocusData {
    pub name: String,
    pub default_url: String,
    pub shiny_url: Option<String>,
    pub primary_type: String,
    pub secondary_type: Option<String>,
    pub has_multiple_forms: bool,
}

#[derive(Clone, Debug)]
enum FocusState {
    Unset,
    Loading,
    Loaded(FocusData),
    Failed(String),
}

fn Focus(cx: Scope) -> Element {
    let focus_state = use_shared_state::<FocusState>(cx)?;

    match &*focus_state.read() {
        FocusState::Unset => render! {"Hover over a pokemon to preview it here"},
        FocusState::Loading => render! {"Loading..."},
        FocusState::Loaded(focus_data) => {
            let serebii_link = format!("https://www.serebii.net/pokedex-sv/{}", focus_data.name);
            render! {
                h1 { class: "text-3xl", "{focus_data.name.clone()}" }
                p { "{focus_data.primary_type.clone()}" }
                if let Some(secondary_type) = focus_data.secondary_type.clone() {
                    rsx!(p {
                        "{secondary_type}"
                    })
                }
                b { "Easy 3 star sparkling/encounter/mark sandwich:" }
                "tomato + onion + green pepper + hamburger + 2 * ({TYPES_INFO.get(focus_data.primary_type.as_str()).unwrap().ingredient}"
                if let Some(secondary_type) = focus_data.secondary_type.clone() {
                    rsx!(" or {TYPES_INFO.get(secondary_type.as_str()).unwrap().ingredient}")
                }
                b { ")" }
                p { a { href: "{serebii_link}", target: "_blank", "Serebii link" } }
                div { display: "flex", flex_direction: "row",
                    img { src: "{focus_data.default_url}", width: "100%" }
                    img {
                        src: "{focus_data.shiny_url.clone().unwrap_or_default()}",
                        width: "100%"
                    }
                }
            }
        }
        FocusState::Failed(err) => render! {"{err}"},
    }
}

async fn load_focus(focus_state: UseSharedState<FocusState>, pokemon_url: String) {
    *focus_state.write() = FocusState::Loading;
    if let Ok(focus_data) = get_data(pokemon_url).await {
        *focus_state.write() = FocusState::Loaded(focus_data.clone());
    } else {
        *focus_state.write() = FocusState::Failed("Failed to load data".to_string());
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FocusItem {
    name: String,
    forms: Vec<Form>,
    sprites: Sprite,
    types: Vec<Type>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Form {
    name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sprite {
    other: Other,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Other {
    #[serde(rename = "official-artwork")]
    official_artwork: OfficialArtwork,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficialArtwork {
    front_default: String,
    front_shiny: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Type {
    #[serde(rename = "type")]
    pokemon_type: PokemonType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PokemonType {
    name: String,
}

async fn get_data(pokemon_url: String) -> Result<FocusData, reqwest::Error> {
    let pokemon: FocusItem = reqwest::get(&pokemon_url).await?.json().await?;
    let default = pokemon.sprites.other.official_artwork.front_default;
    let shiny = pokemon.sprites.other.official_artwork.front_shiny;
    let primary = pokemon.types[0].pokemon_type.name.clone();
    let secondary = pokemon.types.get(1).map(|t| t.pokemon_type.name.clone());

    Ok(FocusData {
        name: pokemon.name,
        default_url: default,
        shiny_url: shiny,
        primary_type: primary,
        secondary_type: secondary,
        has_multiple_forms: pokemon.forms.len() > 1,
    })
}
