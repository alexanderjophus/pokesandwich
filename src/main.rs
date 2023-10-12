#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_storage::use_persistent;
use log::{info, LevelFilter};
use phf::phf_map;
use serde::{Deserialize, Serialize};

pub static BASE_API_URL: &str = "https://pokeapi.co/api/v2/";

const DEXES: [&str; 2] = ["paldea", "kitakami"];
static TYPES_INGREDIENTS: phf::Map<&'static str, &'static str> = phf_map! {
    "normal" => "tofu",
    "grass" => "lettuce",
    "fire" => "red pepper",
    "water" => "cucumber",
    "electric" => "yellow pepper",
    "ice" => "klawf stick",
    "fighting" => "pickle",
    "poison" => "green pepper",
    "ground" => "ham",
    "flying" => "prosciutto",
    "psychic" => "onion",
    "bug" => "cherry tomato",
    "rock" => "bacon",
    "ghost" => "red onion",
    "dragon" => "avocado",
    "dark" => "smoked fillet",
    "steel" => "hamburger",
    "fairy" => "tomato",
};

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/dex/:dex/type/:pokemon_type")]
        DexByType {
            dex: String,
            pokemon_type: String,
        },
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

pub fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

#[inline_props]
fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            display: "flex",
            flex_direction: "row",
            background_color: "grey",
            ul {
                li { Link { to: Route::Home {}, "Home" } }
            }
        }
        Outlet::<Route> {}
    }
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Welcome to the pokemon dex" }
        p { "Select a dex and a type to get started" }
        form {
            display: "flex",
            flex_direction: "row",
            onsubmit: move |event| {
                info!("submitting form: {event:?}");
            },
            select {
                width: "20%",
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
            input { r#type: "submit", },
        }
        Footer {}
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

#[inline_props]
pub fn DexByType(cx: Scope, dex: String, pokemon_type: String) -> Element {
    use_shared_state_provider(cx, || FocusState::Unset);

    cx.render(rsx! {
        script {
            src: "https://kit.fontawesome.com/e04bfc6d26.js",
            crossorigin: "anonymous",
        }
        div {
            display: "flex",
            flex_direction: "row",
            div {
                overflow: "auto",
                max_height: "100vh",
                margin: "10px",
                width: "20%",
                Search {
                    dex: dex.clone(),
                    pokemon_type: pokemon_type.clone(),
                }
            }
            div {
                margin: "10px",
                width: "80%",
                Focus {}
            }
        }
        Footer {}
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
        (Some(Ok(dex)), Some(Ok(types))) => render_dex(cx, dex.clone(), types.clone()),
        (Some(Err(err)), _) => render! {"An error occurred while loading dexes {err}"},
        (_, Some(Err(err))) => render! {"An error occurred while loading types {err}"},
        _ => render! {"Loading items"},
    }
}

fn render_dex(cx: Scope<SearchProps>, dex: Pokedex, types: PokemonTypeResponse) -> Element {
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
            border_collapse: "collapse",
            thead {
                tr {
                    border: "1px solid black",
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
    let fav = use_persistent(cx, &dex_entry.name, || false);

    let fut = use_future(cx, &dex_entry.url, |url| {
        load_focus(focus_state.clone(), url.to_string())
    });
    cx.render(rsx! {
        tr {
            border_bottom: "1px solid black",
            td {
                div {
                    display: "flex",
                    flex_direction: "row",
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
                            fav.set(!fav.get());
                        },
                        i {
                            class: "fa fa-heart",
                            color: if fav.get() {
                                "red"
                            } else {
                                "grey"
                            },
                        }
                    }
                }
            }
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
        FocusState::Unset => render! {
            "Hover over a pokemon to preview it here"
        },
        FocusState::Loading => render! {
            "Loading..."
        },
        FocusState::Loaded(focus_data) => {
            let serebii_link = format!("https://www.serebii.net/pokedex-sv/{}", focus_data.name);
            render! {
                h1 {
                    "{focus_data.name.clone()} - {focus_data.primary_type.clone()} - {focus_data.secondary_type.clone().unwrap_or_default()}"
                }
                b {
                    "Shiny sandwich: tomato + onion + green pepper + hamburger + 2 * ({TYPES_INGREDIENTS.get(focus_data.primary_type.as_str()).unwrap_or(&\"\")}"
                }
                if let Some(secondary_type) = focus_data.secondary_type.clone() {
                    rsx!(b {
                        " or {TYPES_INGREDIENTS.get(secondary_type.as_str()).unwrap_or(&\"\")}"
                    })
                }
                b {
                    ")"
                }
                p {
                    table {
                        border_collapse: "collapse",
                        thead {
                            tr {
                                border: "1px solid black",
                                td {
                                    "serebii"
                                }
                                td {
                                    "has multiple forms"
                                    span {
                                        title: "this may be incorrect, check serebii to be certain",
                                        "⚠️"
                                    }
                                }
                            }
                        }
                        tbody {
                            tr {
                                td {
                                    a {
                                        href: "{serebii_link}",
                                        target: "_blank",
                                        "{focus_data.name.clone()}"
                                    }
                                }
                                td {
                                    if focus_data.has_multiple_forms {
                                        "yes"
                                    } else {
                                        "no"
                                    }
                                }
                            }
                        }
                    }
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
pub struct Type {
    #[serde(rename = "type")]
    pokemon_type: PokemonType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PokemonType {
    name: String,
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

#[inline_props]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}

#[inline_props]
fn Footer(cx: Scope) -> Element {
    render! {
        footer {
            position: "fixed",
            bottom: "0",
            left: "0",
            right: "0",
            height: "50px",
            background_color: "grey",
            color: "white",
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            "Made with "
            span {
                color: "red",
                "❤️"
            }
            " by Alexander Jophus"
            a {
                href: "https://github.com/alexanderjophus",
                target: "_blank",
                i {
                    class: "fa fa-github",
                    font_size: "30px",
                    margin_left: "10px",
                    color: "white",
                }
            }
            a {
                href: "https://twitter.com/alexanderjophus",
                target: "_blank",
                i {
                    class: "fa fa-twitter",
                    font_size: "30px",
                    margin_left: "10px",
                    margin_right: "10px",
                    color: "white",
                }
            }
            "Pokemon data from "
            a {
                href: "https://pokeapi.co/",
                target: "_blank",
                "pokeapi.co"
            }
        }
    }
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(App);
}
