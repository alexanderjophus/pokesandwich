#![allow(non_snake_case)]
use dioxus::prelude::*;
use gloo_console::log;
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
    url: String,
    name: String,
}

fn main() {
    dioxus_web::launch(App);
}

pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || FocusState::Unset);
    let dex = use_state(cx, || "paldea".to_string());
    let pokemon_type = use_state(cx, || "fairy".to_string());

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
            div {
                width: "20%",
                Search {
                    dex: dex.clone(),
                    pokemon_type: pokemon_type.clone(),
                }
            }
            div {
                width: "30%",
                Focus {}
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
        div {
            display: "flex",
            flex_direction: "column",
            width: "100%",
            DexTable {dex_entries: pokedex_entries }
        }
    })
}

#[inline_props]
fn DexTable(cx: Scope, dex_entries: Vec<DexItem>) -> Element {
    cx.render(rsx! {
        table {
            thead {
                tr {
                    th { "id" }
                    th { "name" }
                }
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
    let focus_state = use_shared_state::<FocusState>(cx).unwrap();
    let images = use_ref(cx, || None);

    let fut = use_future(cx, &dex_entry.url, |url| {
        load_focus(images.clone(), focus_state.clone(), url.to_string())
    });

    cx.render(rsx! {
        tr {
            td { "{dex_entry.id}" }
            td { onclick: move |_| {
                fut.restart();
            },
            "{dex_entry.name}" }
        }
    })
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageUrls {
    pub default: String,
    pub shiny: String,
}

#[derive(Clone, Debug)]
enum FocusState {
    Unset,
    Loading,
    Loaded(ImageUrls),
    Failed(String),
}

fn Focus(cx: Scope) -> Element {
    let focus_state = use_shared_state::<FocusState>(cx)?;

    match &*focus_state.read() {
        FocusState::Unset => render! {
            "Hover over a pokemon to preview it here"
        },
        FocusState::Loading => render! {
            "Loading..."
        },
        FocusState::Loaded(urls) => {
            render! {
                div {
                    display: "flex",
                    flex_direction: "row",
                    img {
                        src: "{urls.default}",
                        width: "100%",
                    }
                    img {
                        src: "{urls.shiny}",
                        width: "100%",
                    }
                }
            }
        }
        FocusState::Failed(err) => render! {
            "Failed to load image {err}"
        },
    }
}

async fn load_focus(
    images: UseRef<Option<ImageUrls>>,
    focus_state: UseSharedState<FocusState>,
    pokemon_url: String,
) {
    if let Some(cached) = &*images.read() {
        *focus_state.write() = FocusState::Loaded(cached.clone());
        return;
    }

    *focus_state.write() = FocusState::Loading;
    if let Ok(url) = get_images(pokemon_url).await {
        *focus_state.write() = FocusState::Loaded(url.clone());
        *images.write() = Some(url);
    } else {
        *focus_state.write() = FocusState::Failed("Failed to load image".to_string());
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FocusItem {
    sprites: Sprite,
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
    front_shiny: String,
}

async fn get_images(pokemon_url: String) -> Result<ImageUrls, reqwest::Error> {
    let pokemon: FocusItem = reqwest::get(&pokemon_url)
        .await
        .expect("getting data")
        .json()
        .await
        .expect("parsing json");
    let default = pokemon.sprites.other.official_artwork.front_default;
    let shiny = pokemon.sprites.other.official_artwork.front_shiny;
    Ok(ImageUrls { default, shiny })
}
