use dioxus::prelude::*;
use dioxus_signals::*;
use graphql_client::{GraphQLQuery, Response};
use std::error::Error;

use crate::footer;
use crate::BASE_GRAPHQL_API_URL;

pub fn PokemonFinder(cx: Scope) -> Element {
    cx.render(rsx! {
        div { display: "flex", flex_direction: "row", justify_content: "space-between",
            h1 { "Welcome to the pokemon finder" }
            h1 { "This feature is a work in progress" }
        }
        MovesList {},
        footer::Footer {}
    })
}

#[derive(PartialEq, Props, Clone)]
struct MovesProps {}

fn MovesList(cx: Scope<MovesProps>) -> Element {
    let resp: &UseFuture<Result<moves_and_abilities::ResponseData, Box<dyn Error>>> =
        use_future(cx, &cx.props.clone(), |_| async move {
            let request_body = MovesAndAbilities::build_query(moves_and_abilities::Variables {});

            let gql_addr = BASE_GRAPHQL_API_URL;

            let client = reqwest::Client::new();
            let resp: Response<moves_and_abilities::ResponseData> = client
                .post(format!("{gql_addr}"))
                .json(&request_body)
                .send()
                .await
                .expect("failed to send request")
                .json()
                .await?;

            Ok(resp.data.ok_or("missing response data")?)
        });

    match resp.value() {
        Some(Ok(resp)) => RenderDropdowns(cx, resp.clone()),
        Some(Err(err)) => render! {"An error occurred while loading {err}"},
        _ => render! {"Loading items"},
    }
}

fn RenderDropdowns(cx: Scope<MovesProps>, resp: moves_and_abilities::ResponseData) -> Element {
    let selected_move = use_signal(cx, || "".to_string());
    let selected_ability = use_signal(cx, || "".to_string());
    let moves = resp.pokemon_v2_move;
    let abilities = resp.pokemon_v2_ability;

    cx.render(rsx! {
        div { display: "flex", flex_direction: "row",
            div { margin: "10px", width: "100%", justify_content: "space-evenly",
                div {
                    h2 { "Moves" }
                    select {
                        class: "bg-white font-bold py-2 px-4 rounded",
                        width: "80%",
                        oninput: move |e| {
                            selected_move.set(e.data.value.clone());
                        },
                        for r#move in moves.iter() {
                            option { value: "{r#move.name.clone()}", "{r#move.name.clone()}" }
                        }
                    }
                }
                div {
                    h2 { "Abilities" }
                    select {
                        class: "bg-white font-bold py-2 px-4 rounded",
                        width: "80%",
                        oninput: move |e| {
                            selected_ability.set(e.data.value.clone());
                        },
                        for ability in abilities.iter() {
                            option { value: "{ability.name.clone()}", "{ability.name.clone()}" }
                        }
                    }
                }
            }
            div { overflow: "auto", max_height: "100vh", margin: "10px", width: "100%",
                render!(
                    PokemonList { selected_move: selected_move, selected_ability: selected_ability }
                )
            }
        }
    })
}

#[derive(PartialEq, Props, Clone)]
struct PokemonListProps {
    selected_move: Signal<String>,
    selected_ability: Signal<String>,
}

fn PokemonList(cx: Scope<PokemonListProps>) -> Element {
    let resp: &UseFuture<Result<finder::ResponseData, Box<dyn Error>>> = use_future(
        cx,
        (
            &cx.props.selected_move.to_string(),
            &cx.props.selected_ability.to_string(),
        ),
        |(selected_move, selected_ability)| async move {
            let request_body = Finder::build_query(finder::Variables {
                move_name: selected_move.to_string(),
                ability_name: selected_ability.to_string(),
            });

            let gql_addr = BASE_GRAPHQL_API_URL;

            let client = reqwest::Client::new();
            let resp: Response<finder::ResponseData> = client
                .post(format!("{gql_addr}"))
                .json(&request_body)
                .send()
                .await
                .expect("failed to send request")
                .json()
                .await?;

            Ok(resp.data.ok_or("missing response data")?)
        },
    );

    match resp.value() {
        Some(Ok(resp)) => {
            render!(
                div { display: "flex", flex_direction: "row", flex_wrap: "wrap",
                    for pokemon in resp.pokemon_v2_pokemon.iter() {
                        div { margin: "10px", width: "200px", height: "200px", border: "1px solid black",
                            div { display: "flex", flex_direction: "row", justify_content: "space-between",
                                h2 { "{pokemon.name.clone()}" }
                            }
                            img { src: "{pokemon.pokemon_v2_pokemonsprites[0]
                                .sprites
                                .get(\"other\")
                                .unwrap()
                                .get(\"official-artwork\")
                                .unwrap()
                                .get(\"front_default\")
                                .unwrap()
                                .to_string()
                                .trim_matches('\"')}" }
                        }
                    }
                }
            )
        }
        Some(Err(err)) => render! {"An error occurred while loading {err}"},
        _ => render! {"Loading items"},
    }
}

#[allow(non_camel_case_types)]
type jsonb = serde_json::Map<String, serde_json::Value>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graph/schema.graphql",
    query_path = "graph/query.graphql",
    response_derives = "PartialEq, Clone, Default, Debug, Serialize, Deserialize"
)]
pub struct MovesAndAbilities;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graph/schema.graphql",
    query_path = "graph/query.graphql",
    response_derives = "PartialEq, Clone, Default, Debug, Serialize, Deserialize"
)]
pub struct Finder;
