use dioxus::prelude::*;
use dioxus_storage::use_persistent;
use graphql_client::{GraphQLQuery, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;

use crate::consts::{BASE_API_URL, TYPES_INFO};

#[component]
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
    let pokemon = use_future(cx, &cx.props.clone(), |_| {
        let variables = poke_api_pokemon::Variables {
            dex: cx.props.dex.to_string(),
            pokemon_type: cx.props.pokemon_type.to_string(),
        };
        perform_gql_query(variables)
    });

    match pokemon.value() {
        Some(Ok(pokemon)) => RenderDex(cx, pokemon.clone()),
        Some(Err(err)) => render! {"An error occurred while loading {err}"},
        _ => render! {"Loading items"},
    }
}

fn RenderDex(
    cx: Scope<SearchProps>,
    pokemon: Vec<poke_api_pokemon::PokeApiPokemonPokemonV2Pokemon>,
) -> Element {
    cx.render(rsx! {
        div { overflow: "hidden", background_color: TYPES_INFO.get(cx.props.pokemon_type.as_str()).unwrap().color, border_radius: "50%", width: "100px", height: "100px", img { src: "/icons/{cx.props.pokemon_type.clone()}.svg" } }
        div { overflow: "auto", display: "flex", flex_direction: "column", width: "100%", DexTable { pokemon: pokemon } }
    })
}

#[component]
fn DexTable(cx: Scope, pokemon: Vec<poke_api_pokemon::PokeApiPokemonPokemonV2Pokemon>) -> Element {
    cx.render(rsx! {
        table { border_collapse: "collapse",
            thead {
                tr { th { "Pokemon Name" } }
            }
            tbody {
                for entry in pokemon.iter() {
                    DexRow { entry: entry.clone() }
                }
            }
        }
    })
}

#[component]
fn DexRow(cx: Scope, entry: poke_api_pokemon::PokeApiPokemonPokemonV2Pokemon) -> Element {
    let fav = use_persistent(cx, "faves", || HashSet::new());
    let focus_state = use_shared_state::<FocusState>(cx).unwrap();

    cx.render(rsx! {
        tr { class: "border-2 hover:bg-gray-100 hover:ring-2 hover:ring-pink-500 hover:ring-inset",
            td {
                div { display: "flex", flex_direction: "row",
                    div {
                        width: "80%",
                        onclick: move |_event| {
                            load_focus(focus_state.clone(), entry.clone())
                        },
                        "{entry.name}"
                    }
                    div {
                        width: "20%",
                        onclick: move |_| {
                            if fav.get().contains(&entry.name) {
                                fav.modify(|faves| {
                                    faves.remove(&entry.name);
                                });
                            } else {
                                fav.modify(|faves| {
                                    faves.insert(entry.name.clone());
                                });
                            }
                        },
                        i { class: "fa fa-heart", color: if fav.get().contains(&entry.name) { "red" } else { "grey" } }
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct FocusData {
    pub name: String,
    pub default_url: String,
    pub shiny_url: Option<String>,
    pub types: Vec<String>,
    pub capture_rate: i64,
}

#[derive(Clone)]
enum FocusState {
    Unset,
    Loading,
    Loaded(FocusData),
    Failed(String),
}

fn Focus(cx: Scope) -> Element {
    let focus_state = use_shared_state::<FocusState>(cx)?;

    match &*focus_state.read() {
        FocusState::Unset => render! {"Click on a pokemon to preview it here"},
        FocusState::Loading => render! {"Loading..."},
        FocusState::Loaded(focus_data) => {
            let serebii_link = format!("https://www.serebii.net/pokedex-sv/{}", focus_data.name);
            render! {
                h1 { class: "text-3xl", "{focus_data.name.clone()}" }
                div { display: "flex", flex_direction: "row",
                    focus_data.types.join(" + ")
                }
                b { "Easy 3 star sparkling/encounter/title sandwich:" }
                "tomato + onion + green pepper + hamburger + 2 * ("
                    focus_data.types.iter().map(|t| TYPES_INFO.get(t.as_str()).unwrap().ingredient).collect::<Vec<&str>>().join(" or ")
                ")"
                p {
                    a { href: "{serebii_link}", target: "_blank", "Serebii" }
                    " | Capture Rate: {focus_data.capture_rate}"
                }
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

async fn load_focus(
    focus_state: UseSharedState<FocusState>,
    pokemon: poke_api_pokemon::PokeApiPokemonPokemonV2Pokemon,
) {
    *focus_state.write() = FocusState::Loading;
    if let Ok(focus_data) = get_data(pokemon.clone()).await {
        *focus_state.write() = FocusState::Loaded(focus_data.clone());
    } else {
        *focus_state.write() = FocusState::Failed("Failed to load data".to_string());
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graph/schema.graphql",
    query_path = "graph/query.graphql",
    response_derives = "PartialEq, Clone, Default, Debug, Serialize, Deserialize"
)]
pub struct PokeApiPokemon;

#[warn(non_camel_case_types)]
type jsonb = serde_json::Map<String, serde_json::Value>;

async fn perform_gql_query(
    variables: poke_api_pokemon::Variables,
) -> Result<Vec<poke_api_pokemon::PokeApiPokemonPokemonV2Pokemon>, Box<dyn Error>> {
    let request_body = PokeApiPokemon::build_query(variables);

    let gql_addr = BASE_API_URL;

    let client = reqwest::Client::new();
    let resp: Response<poke_api_pokemon::ResponseData> = client
        .post(format!("{gql_addr}"))
        .json(&request_body)
        .send()
        .await
        .expect("failed to send request")
        .json()
        .await?;

    Ok(resp.data.ok_or("missing response data")?.pokemon_v2_pokemon)
}

#[derive(Deserialize, Default)]
struct SpriteResp {
    sprites: Sprites,
}

#[derive(Deserialize, Default)]
struct Sprites {
    other: Other,
}

#[derive(Deserialize, Default)]
struct Other {
    #[serde(rename = "official-artwork")]
    official_artwork: OfficialArtwork,
}

#[derive(Deserialize, Default)]
struct OfficialArtwork {
    front_default: String,
    front_shiny: Option<String>,
}

async fn get_data(
    pokemon: poke_api_pokemon::PokeApiPokemonPokemonV2Pokemon,
) -> Result<FocusData, reqwest::Error> {
    let capture_rate = pokemon
        .pokemon_v2_pokemonspecy
        .clone()
        .unwrap_or_default()
        .capture_rate
        .unwrap_or_default();

    // wtf?
    let sprites = &pokemon.pokemon_v2_pokemonsprites[0];
    let sprites = serde_json::to_string(&sprites).unwrap_or_default();
    let sprites: SpriteResp = serde_json::from_str(&sprites).unwrap_or_default();

    let types = pokemon
        .pokemon_v2_pokemontypes
        .iter()
        .map(|t| t.pokemon_v2_type.clone().unwrap_or_default().name.clone())
        .collect();

    Ok(FocusData {
        name: pokemon.name,
        default_url: sprites.sprites.other.official_artwork.front_default,
        shiny_url: sprites.sprites.other.official_artwork.front_shiny,
        types,
        capture_rate,
    })
}
