use dioxus::prelude::*;
use graphql_client::{GraphQLQuery, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::consts::{BASE_API_URL, TYPES_INFO};

#[derive(Clone)]
pub enum FocusState {
    Unset,
    Loading,
    Loaded(FocusData),
    Failed(String),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct FocusData {
    name: String,
    default_url: String,
    shiny_url: Option<String>,
    types: Vec<String>,
    capture_rate: i64,
}

pub fn Focus(cx: Scope) -> Element {
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

pub async fn load_focus(
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

#[allow(non_camel_case_types)]
type jsonb = serde_json::Map<String, serde_json::Value>;

pub async fn perform_gql_query(
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
