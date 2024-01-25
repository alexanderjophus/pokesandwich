use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::FaSquareCaretDown;
use dioxus_free_icons::Icon;
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
    let name = use_signal(cx, || "".to_string());

    let selected_moves = use_signal(cx, || Vec::new());
    let mut moves = resp
        .pokemon_v2_move
        .iter()
        .map(|m| m.name.clone())
        .collect::<Vec<_>>();
    moves.sort();
    let moves_searchable = use_state(cx, || false);

    let selected_abilities = use_signal(cx, || Vec::new());
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

    let selected_types = use_signal(cx, || Vec::new());
    let mut types = resp
        .pokemon_v2_type
        .iter()
        .map(|t| t.name.clone())
        .collect::<Vec<_>>();
    types.sort();
    let types_searchable = use_state(cx, || false);

    cx.render(rsx! {
        div { display: "flex", flex_direction: "row",
            div { margin: "10px", width: "100%", justify_content: "space-evenly",
                div {
                    input {
                        class: "bg-white focus:outline-none focus:shadow-outline border border-gray-300 rounded-lg py-2 px-4 block w-full appearance-none leading-normal",
                        r#type: "text",
                        placeholder: "Search",
                        oninput: move |e| {
                            name.set(e.data.value.clone());
                        }
                    }
                }
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
                                selected_moves.set(Vec::new());
                            }
                            moves_searchable.set(!moves_searchable.get().clone());
                        }
                    }
                    label {
                        class: "block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4",
                        r#for: "moves", "Moves"
                    }
                    if moves_searchable.get().clone() {
                        rsx!( SearchableDropdown { selected_options: selected_moves, items: moves.clone() } )
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
                                selected_abilities.set(Vec::new());
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
                            SearchableDropdown { selected_options: selected_abilities, items: ability_keys.clone() }
                            // {
                            //     if let Some(description) = abilities.get(&selected_ability.read().to_string()) {
                            //         rsx!( p { "{description}" } )
                            //     } else {
                            //         rsx!( p { "No description available" } )
                            //     }
                            // }
                        )
                    }
                }
                div {
                    input {
                        class: "mr-2 leading-tight",
                        r#type: "checkbox",
                        id: "types",
                        name: "types",
                        value: "types",
                        checked: "false",
                        onclick: move |_| {
                            if types_searchable.get().clone() {
                                selected_types.set(Vec::new());
                            }
                            types_searchable.set(!types_searchable.get().clone());
                        }
                    }
                    label {
                        class: "block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4",
                        r#for: "types", "Types"
                    }
                    if types_searchable.get().clone() {
                        rsx!( SearchableDropdown { selected_options: selected_types, items: types.clone() } )
                    }
                }
            }
            div { overflow: "auto", max_height: "100vh", margin: "10px", width: "100%",
                render!(
                    PokemonList { name: name, selected_moves: selected_moves, selected_abilities: selected_abilities, selected_types: selected_types }
                )
            }
        }
    })
}

#[derive(PartialEq, Props, Clone)]
struct SearchableDropdownProps<T: std::fmt::Display + 'static + Clone + std::cmp::PartialEq> {
    selected_options: Signal<Vec<T>>,
    items: Vec<T>,
}

fn SearchableDropdown<T: std::fmt::Display + 'static + Clone + std::cmp::PartialEq>(
    cx: Scope<SearchableDropdownProps<T>>,
) -> Element {
    let toggled = use_state(cx, || false);
    let search_text = use_state(cx, || "".to_string());

    cx.render(rsx! {
        div { margin: "10px", width: "100%", justify_content: "space-evenly",
            button {
                class: "bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow",
                onclick: move |_| {
                    toggled.set(!toggled.get().clone());
                },
                span {
                    class: "mr-2",
                    "Select an option"
                    Icon {
                        icon: FaSquareCaretDown,
                        width: 20,
                        height: 20,
                    }
                }
            }
            if toggled.get().clone() {
                rsx!( div {
                    class: "absolute mt-1 w-full rounded-md bg-white shadow-lg",
                    input {
                        class: "block w-full px-4 py-2 text-gray-800 border rounded-md  border-gray-300 focus:outline-none",
                        r#type: "text",
                        placeholder: "Search items",
                        autocomplete: "off",
                        oninput: move |e| {
                            search_text.set(e.data.value.clone());
                        }
                    }
                    ul {
                        class: "overflow-auto max-h-60 rounded-md py-1 text-base ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm",
                        for item in cx.props.items.iter() {
                            if item.to_string().to_lowercase().contains(&search_text.get().to_lowercase()) {
                                rsx!( li {
                                    class: "text-gray-900 cursor-default select-none relative py-2 pl-3 pr-9",
                                    div {
                                        class: "flex items-center",
                                        input {
                                            class: "absolute cursor-pointer opacity-0 h-0 w-0",
                                            r#type: "checkbox",
                                            id: "{item.clone().to_string()}",
                                            name: "{item.clone().to_string()}",
                                            value: "{item.clone().to_string()}",
                                            // checked: "{cx.props.selected.read().to_string() == item.clone().to_string()}",
                                            onclick: move |_| {
                                                let mut selected = cx.props.selected_options.read().clone();
                                                if selected.contains(&item.clone()) {
                                                    selected.retain(|x| x != &item.clone());
                                                } else {
                                                    selected.push(item.clone());
                                                }
                                                cx.props.selected_options.set(selected);
                                            }
                                        }
                                        label {
                                            class: "ml-3 block font-normal truncate",
                                            r#for: "{item.clone().to_string()}", "{item.clone().to_string()}"
                                        }
                                    }
                                })
                            }
                        }
                    }
                })
            }
        }
    })
}

#[derive(PartialEq, Props, Clone)]
struct PokemonListProps {
    name: Signal<String>,
    selected_moves: Signal<Vec<String>>,
    selected_abilities: Signal<Vec<String>>,
    selected_types: Signal<Vec<String>>,
}

fn PokemonList(cx: Scope<PokemonListProps>) -> Element {
    let resp: &UseFuture<Result<finder::ResponseData, Box<dyn Error>>> = use_future(
        cx,
        (
            &cx.props.name.to_string(),
            &cx.props.selected_moves.read().join("|"),
            &cx.props.selected_abilities.read().join("|"),
            &cx.props.selected_types.read().join("|"),
        ),
        |(name, selected_moves, selected_abilities, selected_types)| async move {
            let request_body = Finder::build_query(finder::Variables {
                name: name.to_string(),
                move_name: format!("({})", selected_moves.to_string()),
                ability_name: format!("({})", selected_abilities.to_string()),
                type_name: format!("({})", selected_types.to_string()),
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
