use charming::component::{RadarCoordinate, Title};
use charming::series::Radar;
use charming::{Chart, WasmRenderer};
use dioxus::prelude::*;
use graphql_client::{GraphQLQuery, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::shiny_dex::TYPES_INFO;
use crate::BASE_GRAPHQL_API_URL;

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
    stats: Vec<i64>,
    default_url: String,
    shiny_url: Option<String>,
    types: Vec<String>,
    capture_rate: i64,
}

#[component]
pub fn Focus(focus_state: ReadOnlySignal<FocusState>) -> Element {
    match &*focus_state.read() {
        FocusState::Unset => rsx! {"Click on a pokemon to preview it here"},
        FocusState::Loading => rsx! {"Loading..."},
        FocusState::Loaded(focus_data) => {
            rsx! { FocusDetail { focus_data: focus_data.clone() } }
        }
        FocusState::Failed(err) => rsx! {"{err}"},
    }
}

#[component]
fn FocusDetail(focus_data: ReadOnlySignal<FocusData>) -> Element {
    let mut chain = use_signal(|| 0);
    let mut sandwich = use_signal(|| 0);
    let mut shiny_charm = use_signal(|| false);
    let (odds, rolls) = shiny_odds(
        chain.read().clone(),
        sandwich.read().clone(),
        shiny_charm.read().clone(),
    );

    let serebii_link = format!(
        "https://www.serebii.net/pokedex-sv/{}",
        focus_data.read().name
    );

    let renderer = use_signal(|| WasmRenderer::new(320, 320));

    use_effect(move || {
        let chart = Chart::new()
            .title(Title::new().text("Base Stats"))
            .radar(RadarCoordinate::new().indicator(vec![
                ("HP", 1, 255),
                ("Attack", 1, 255),
                ("Defence", 1, 255),
                ("Sp Attack", 1, 255),
                ("Sp Defence", 1, 255),
                ("Speed", 1, 255),
            ]))
            .series(
                Radar::new()
                    .name("Base stats")
                    .data(vec![(focus_data.read().stats.clone(), "Base Stats")]),
            );
        renderer.read_unchecked().render("chart", &chart).unwrap();
    });

    rsx! {
        div { display: "flex", flex_direction: "row",
            div { margin: "10px", width: "50%",
                h1 { class: "text-3xl", "{focus_data.read().name.clone()}" }
                div { display: "flex", flex_direction: "row", margin: "10px",
                    for t in focus_data.read().types.iter() {
                        "{t}"
                    }
                }
                b { "Easy 3 star sparkling/encounter/title sandwich:" }
                p {
                    "tomato + onion + green pepper + hamburger + 2 * ({focus_data.read().types.iter().map(|t| TYPES_INFO.get(t.as_str()).unwrap().ingredient).collect::<Vec<&str>>().join(\" or \")})"
                }
                p {
                    a { href: "{serebii_link}", target: "_blank", "Serebii" }
                    " | Capture Rate: {focus_data.read().capture_rate}"
                }
            }
            div {
              id: "chart",
              style: "display: inline-block; height: 400px; width: 600px;",
            }
            div { margin: "10px", width: "50%",
                p {
                    "Shiny Charm: ",
                    input {
                        r#type: "checkbox",
                        oninput: move |_| {
                            let set = shiny_charm.read().clone();
                            shiny_charm.set(!set);
                        },
                        checked: *shiny_charm.read()
                    }
                }
                p {
                    "Sandwich Level: ",
                    select {
                        oninput: move |e| {
                            sandwich.set(e.data.value().parse::<i64>().unwrap_or_default());
                        },
                        for i in [0, 1, 3] {
                            option { value: "{i}", b { "{i}" } }
                        }
                    }
                }
                p {
                    button {
                        style: "margin-right: 10px;",
                        onclick: move |_| {
                            chain += 1;
                        },
                        "Chain +"
                    }
                    b {"{chain}" },
                    button {
                        style: "margin-left: 10px;",
                        onclick: move |_| {
                            chain *= 0;
                        },
                        "Chain Reset"
                    }
                }
                p {
                    "Odds:"
                    b { "{odds * 100.0:.3}%" }
                    " | Rolls:"
                    b { "{rolls}" }
                    " of 4096"
                }
            }
        }
        div { display: "flex", flex_direction: "row",
            img { src: "{focus_data.read().default_url}", width: "100%" }
            img {
                src: "{focus_data.read().shiny_url.clone().unwrap_or_default()}",
                width: "100%"
            }
        }
    }
}

fn shiny_odds(chain: i64, sandwich_level: i64, shiny_charm: bool) -> (f64, i64) {
    let mut rolls = sandwich_level + 1;
    match chain {
        30..=59 => rolls += 1,
        60..=i64::MAX => rolls += 2,
        _ => {}
    };
    if shiny_charm {
        rolls += 2;
    };
    let odds = 1.0 - (4095.0 / 4096.0f64).powi(rolls as i32);
    (odds, rolls)
}

pub async fn load_focus(
    mut focus_state: Signal<FocusState>,
    pokemon: ReadOnlySignal<dex_by_type::DexByTypePokemonV2Pokemon>,
) {
    *focus_state.write() = FocusState::Loading;
    if let Ok(focus_data) = get_data(pokemon()).await {
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
pub struct DexByType;

#[allow(non_camel_case_types)]
type jsonb = serde_json::Map<String, serde_json::Value>;

pub async fn perform_gql_query(
    variables: dex_by_type::Variables,
) -> Result<Vec<dex_by_type::DexByTypePokemonV2Pokemon>, Box<dyn Error>> {
    let request_body = DexByType::build_query(variables);

    let gql_addr = BASE_GRAPHQL_API_URL;

    let client = reqwest::Client::new();
    let resp: Response<dex_by_type::ResponseData> = client
        .post(format!("{gql_addr}"))
        .json(&request_body)
        .send()
        .await
        .expect("failed to send request")
        .json()
        .await?;

    Ok(resp.data.ok_or("missing response data")?.pokemon_v2_pokemon)
}

async fn get_data(
    pokemon: dex_by_type::DexByTypePokemonV2Pokemon,
) -> Result<FocusData, reqwest::Error> {
    let capture_rate = pokemon
        .pokemon_v2_pokemonspecy
        .clone()
        .unwrap_or_default()
        .capture_rate
        .unwrap_or_default();

    let sprites = &pokemon.pokemon_v2_pokemonsprites[0]
        .sprites
        .get("other")
        .unwrap()
        .get("official-artwork")
        .unwrap();

    let types = pokemon
        .pokemon_v2_pokemontypes
        .iter()
        .map(|t| t.pokemon_v2_type.clone().unwrap_or_default().name.clone())
        .collect();

    Ok(FocusData {
        name: pokemon.name,
        stats: pokemon
            .pokemon_v2_pokemonstats
            .iter()
            .map(|s| s.base_stat)
            .collect(),
        default_url: sprites
            .get("front_default")
            .unwrap()
            .to_string()
            .trim_matches('\"')
            .to_string(),
        shiny_url: sprites
            .get("front_shiny")
            .map(|s| s.to_string().trim_matches('\"').to_string()),
        types,
        capture_rate,
    })
}
