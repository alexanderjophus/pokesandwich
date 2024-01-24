use dioxus::prelude::*;
use dioxus_signals::*;
use graphql_client::{GraphQLQuery, Response};
use std::collections::BTreeMap;
use std::error::Error;

use crate::footer;
use crate::BASE_GRAPHQL_API_URL;

pub fn PokemonFinder(cx: Scope) -> Element {
    cx.render(rsx! {
        div { display: "flex", flex_direction: "row", justify_content: "space-between",
            h1 { "Welcome to the pokemon finder" }
            h1 { "This feature is a work in progress" }
        }
        FiltersList {},
        footer::Footer {}
    })
}

#[derive(PartialEq, Props, Clone)]
struct FiltersListProps {}

fn FiltersList(cx: Scope<FiltersListProps>) -> Element {
    let resp: &UseFuture<Result<filters::ResponseData, Box<dyn Error>>> =
        use_future(cx, &cx.props.clone(), |_| async move {
            let request_body = Filters::build_query(filters::Variables {});

            let gql_addr = BASE_GRAPHQL_API_URL;

            let client = reqwest::Client::new();
            let resp: Response<filters::ResponseData> = client
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
        _ => render! {"Loading filters"},
    }
}

fn RenderDropdowns(cx: Scope<FiltersListProps>, resp: filters::ResponseData) -> Element {
    let selected_move = use_signal(cx, || "".to_string());
    let mut moves = resp
        .pokemon_v2_move
        .iter()
        .map(|m| m.name.clone())
        .collect::<Vec<_>>();
    moves.sort();
    let moves_searchable = use_state(cx, || false);

    let selected_ability = use_signal(cx, || "".to_string());
    let abilities = resp
        .pokemon_v2_ability
        .iter()
        .map(|a| {
            let mut description = "".to_string();
            if let Some(effect) = a.pokemon_v2_abilityflavortexts.get(0) {
                description = effect.flavor_text.to_string();
            }
            (a.name.clone(), description)
        })
        .collect::<BTreeMap<_, _>>();
    let mut ability_keys = abilities.keys().cloned().collect::<Vec<_>>();
    ability_keys.sort();
    let abilities_searchable = use_state(cx, || false);

    cx.render(rsx! {
        div { display: "flex", flex_direction: "row",
            div { margin: "10px", width: "100%", justify_content: "space-evenly",
                div {
                    input {
                        class: "mr-2 leading-tight",
                        r#type: "checkbox",
                        id: "moves",
                        name: "moves",
                        value: "moves",
                        checked: "false",
                        onclick: move |_| {
                            if moves_searchable.get().clone() {
                                selected_move.set("".to_string());
                            }
                            moves_searchable.set(!moves_searchable.get().clone());
                        }
                    }
                    label {
                        class: "block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4",
                        r#for: "moves", "Moves"
                    }
                    if moves_searchable.get().clone() {
                        rsx!( SearchableDropdown { selected: selected_move, items: moves.clone() } )
                    }
                }
                div {
                    input {
                        class: "mr-2 leading-tight",
                        r#type: "checkbox",
                        id: "abilities",
                        name: "abilities",
                        value: "abilities",
                        checked: "false",
                        onclick: move |_| {
                            if abilities_searchable.get().clone() {
                                selected_ability.set("".to_string());
                            }
                            abilities_searchable.set(!abilities_searchable.get().clone());
                        }
                    }
                    label {
                        class: "block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4",
                        r#for: "abilities", "Abilities"
                    }
                    if abilities_searchable.get().clone() {
                        rsx!(
                            SearchableDropdown { selected: selected_ability, items: ability_keys.clone() }
                            {
                                if let Some(description) = abilities.get(&selected_ability.read().to_string()) {
                                    rsx!( p { "{description}" } )
                                } else {
                                    rsx!( p { "No description available" } )
                                }
                            }
                        )
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
struct SearchableDropdownProps<T: std::fmt::Display> {
    selected: Signal<String>,
    items: Vec<T>,
}

fn SearchableDropdown<T: std::fmt::Display>(cx: Scope<SearchableDropdownProps<T>>) -> Element {
    let search_text = use_state(cx, || "".to_string());
    let filtered_items = cx
        .props
        .items
        .iter()
        .filter(|&item| {
            item.to_string()
                .to_lowercase()
                .contains(&search_text.get().to_lowercase())
        })
        .collect::<Vec<_>>();

    cx.render(rsx! {
        div { display: "flex", flex_direction: "row",
            div { margin: "10px", width: "100%", justify_content: "space-evenly",
                div {
                    input {
                        class: "bg-white focus:outline-none focus:shadow-outline border border-gray-300 rounded-lg py-2 px-4 block w-full appearance-none leading-normal",
                        r#type: "text",
                        placeholder: "Search",
                        oninput: move |e| {
                            search_text.set(e.data.value.clone());
                        }
                    }
                }
                div {
                    select {
                        class: "bg-white font-bold py-2 px-4 rounded",
                        width: "80%",
                        oninput: move |e| {
                            cx.props.selected.set(e.data.value.clone());
                        },
                        option { disabled: "true", selected: "true", value: "", "Select an option" }
                        for item in filtered_items.iter() {
                            option { value: "{item.clone()}", "{item.clone()}" }
                        }
                    }
                }
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
                            position: "relative",
                            img { src: "{pokemon.pokemon_v2_pokemonsprites[0]
                                .sprites
                                .get(\"other\")
                                .unwrap()
                                .get(\"official-artwork\")
                                .unwrap()
                                .get(\"front_default\")
                                .unwrap()
                                .to_string()
                                .trim_matches('\"')}"
                            }
                            div {
                                display: "flex",
                                flex_direction: "row",
                                justify_content: "space-between",
                                position: "absolute",
                                bottom: "0",
                                width: "100%",
                                background: "rgba(0, 0, 0, 0.5)",
                                color: "white",
                                font_size: "20px",
                                text_align: "center",
                                transition: ".5s ease",
                                opacity: "0.5",
                                "{pokemon.name.clone()}"
                            }
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
pub struct Filters;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graph/schema.graphql",
    query_path = "graph/query.graphql",
    response_derives = "PartialEq, Clone, Default, Debug, Serialize, Deserialize"
)]
pub struct Finder;
