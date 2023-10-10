#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub static BASE_API_URL: &str = "https://pokeapi.co/api/v2/";

const DEXES: [&str; 2] = ["paldea", "kitakami"];
const TYPES: [&str; 18] = [
    "normal", "fighting", "flying", "poison", "ground", "rock", "bug", "ghost", "steel", "fire",
    "water", "grass", "electric", "psychic", "ice", "dragon", "dark", "fairy",
];

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
    let pokemon_type = use_state(cx, || "normal".to_string());
    let images: &UseRef<Option<FocusData>> = use_ref(cx, || None);

    cx.render(rsx! {
        select {
            onchange: move |e| {
                images.needs_update();
                dex.set(e.data.value.clone());
            },
            for dex in &DEXES {
                option {
                    value: *dex,
                    "{dex}"
                }
            }
        }
        select {
            onchange: move |e| {
                images.needs_update();
                pokemon_type.set(e.data.value.clone());
            },
            for pokemon_type in &TYPES {
                option {
                    value: *pokemon_type,
                    "{pokemon_type}"
                }
            }
        }
        div {
            display: "flex",
            flex_direction: "row",
            div {
                overflow: "auto",
                max_height: "100vh",
                width: "20%",
                Search {
                    dex: dex.clone(),
                    pokemon_type: pokemon_type.clone(),
                }
            }
            div {
                width: "80%",
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
    let dexes_future = use_future(cx, &cx.props.clone(), |_| {
        get_pokedex(cx.props.dex.to_string())
    });
    let pokemon_types = use_future(cx, &cx.props.clone(), |_| {
        get_types(cx.props.pokemon_type.to_string())
    });

    match (dexes_future.value(), pokemon_types.value()) {
        (Some(Ok(dex)), Some(Ok(types))) => render_dex(cx, dex.clone(), types.clone()),
        (Some(Err(err)), _) => render! {"An error occurred while loading dexes {err}"},
        (_, Some(Err(err))) => render! {"An error occurred while loading types {err}"},
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

    let fut = use_future(cx, &dex_entry.url, |url| {
        load_focus(focus_state.clone(), url.to_string())
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
pub struct FocusData {
    pub name: String,
    pub default_url: String,
    pub shiny_url: Option<String>,
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
        FocusState::Unset => render! {
            "Hover over a pokemon to preview it here"
        },
        FocusState::Loading => render! {
            "Loading..."
        },
        FocusState::Loaded(focus_data) => {
            render! {
                div {
                    "{focus_data.name.clone()}"
                }
                div {
                    display: "flex",
                    flex_direction: "row",
                    img {
                        src: "{focus_data.default_url}",
                        width: "100%",
                    }
                    img {
                        src: "{focus_data.shiny_url.clone().unwrap_or_default()}",
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

async fn load_focus(focus_state: UseSharedState<FocusState>, pokemon_url: String) {
    *focus_state.write() = FocusState::Loading;
    if let Ok(url) = get_images(pokemon_url).await {
        *focus_state.write() = FocusState::Loaded(url.clone());
    } else {
        *focus_state.write() = FocusState::Failed("Failed to load image".to_string());
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FocusItem {
    name: String,
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
    front_shiny: Option<String>,
}

async fn get_images(pokemon_url: String) -> Result<FocusData, reqwest::Error> {
    let pokemon: FocusItem = reqwest::get(&pokemon_url).await?.json().await?;
    let default = pokemon.sprites.other.official_artwork.front_default;
    let shiny = pokemon.sprites.other.official_artwork.front_shiny;
    Ok(FocusData {
        name: pokemon.name,
        default_url: default,
        shiny_url: shiny,
    })
}
