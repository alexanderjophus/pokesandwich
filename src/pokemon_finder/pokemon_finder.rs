use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::FaSquareCaretDown;
use dioxus_free_icons::Icon;
use graphql_client::{GraphQLQuery, Response};
use std::collections::BTreeMap;

use crate::footer;
use crate::BASE_GRAPHQL_API_URL;

pub fn PokemonFinder() -> Element {
    let name = use_signal(|| "".to_string());
    let selected_moves = use_signal(|| Vec::new());
    let selected_abilities = use_signal(|| Vec::new());
    let selected_types = use_signal(|| Vec::new());

    rsx! {
        div { display: "flex", flex_direction: "row", justify_content: "space-between",
            h1 { "Welcome to the pokemon finder" }
        }
        div { display: "flex", flex_direction: "row",
            div { margin: "10px", width: "25%", justify_content: "space-evenly",
                FiltersList {
                    name: name,
                    selected_moves: selected_moves,
                    selected_abilities: selected_abilities,
                    selected_types: selected_types,
                },
            },
            div { overflow: "auto", max_height: "100vh", margin: "10px", width: "75%",
                PokemonList {
                    name: name,
                    selected_moves: selected_moves,
                    selected_abilities: selected_abilities,
                    selected_types: selected_types
                }
            },
        },
        footer::Footer {}
    }
}

#[derive(PartialEq, Props, Clone)]
struct FiltersListProps {
    name: Signal<String>,
    selected_moves: Signal<Vec<String>>,
    selected_abilities: Signal<Vec<String>>,
    selected_types: Signal<Vec<String>>,
}

fn FiltersList(props: FiltersListProps) -> Element {
    let resp = use_resource(|| async move {
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
            .await
            .expect("failed to parse response");

        resp.data.expect("missing response data")
    });

    match &*resp.read_unchecked() {
        Some(resp) => rsx!(RenderDropdowns {
            name: props.name.clone(),
            selected_moves: props.selected_moves.clone(),
            selected_abilities: props.selected_abilities.clone(),
            selected_types: props.selected_types.clone(),
            resp: resp.clone(),
        }),
        _ => rsx! {"Loading filters"},
    }
}

#[derive(PartialEq, Props, Clone)]
struct RenderDropdownsProps {
    name: Signal<String>,
    selected_moves: Signal<Vec<String>>,
    selected_abilities: Signal<Vec<String>>,
    selected_types: Signal<Vec<String>>,
    resp: filters::ResponseData,
}

fn RenderDropdowns(mut props: RenderDropdownsProps) -> Element {
    let mut moves = props
        .resp
        .pokemon_v2_move
        .iter()
        .map(|m| m.name.clone())
        .collect::<Vec<_>>();
    moves.sort();
    let mut moves_searchable = use_signal(|| false);

    let abilities = props
        .resp
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
    let mut abilities_searchable = use_signal(|| false);

    let mut types = props
        .resp
        .pokemon_v2_type
        .iter()
        .map(|t| t.name.clone())
        .collect::<Vec<_>>();
    types.sort();
    let mut types_searchable = use_signal(|| false);

    rsx! {
        div {
            input {
                class: "bg-white w-full focus:outline-none focus:shadow-outline border border-gray-300 rounded-lg py-2 px-4 block appearance-none leading-normal",
                r#type: "text",
                placeholder: "Search",
                oninput: move |e| {
                    props.name.set(e.data.value().clone());
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
                    if moves_searchable.read().clone() {
                        props.selected_moves.set(Vec::new());
                    }
                    let searchable = moves_searchable.read().clone();
                    moves_searchable.set(!searchable);
                }
            }
            label {
                class: "text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4",
                r#for: "moves", "Moves"
            }
            if moves_searchable.read().clone() {
                SearchableDropdown { selected_options: props.selected_moves, items: moves.clone() }
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
                    if abilities_searchable.read().clone() {
                        props.selected_abilities.set(Vec::new());
                    }
                    let searchable = abilities_searchable.read().clone();
                    abilities_searchable.set(!searchable);
                }
            }
            label {
                class: "text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4",
                r#for: "abilities", "Abilities"
            }
            if abilities_searchable.read().clone() {
                SearchableDropdown { selected_options: props.selected_abilities, items: ability_keys.clone() }
                for selected_ability in props.selected_abilities.read().iter() {
                    if let Some(description) = abilities.get(&selected_ability.to_string()) {
                         p { b { "{selected_ability}" } " - {description}" }
                    } else {
                        p { "No description available" }
                    }
                }
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
                    if types_searchable.read().clone() {
                        props.selected_types.set(Vec::new());
                    }
                    let searchable = types_searchable.read().clone();
                    types_searchable.set(!searchable);
                }
            }
            label {
                class: "text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4",
                r#for: "types", "Types"
            }
            if types_searchable.read().clone() {
                SearchableDropdown { selected_options: props.selected_types, items: types.clone(), limit: 2 }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct SearchableDropdownProps<T: std::fmt::Display + 'static + Clone + std::cmp::PartialEq> {
    selected_options: Signal<Vec<T>>,
    items: ReadOnlySignal<Vec<T>>,

    limit: Option<usize>,
}

fn SearchableDropdown(mut props: SearchableDropdownProps<String>) -> Element {
    let mut toggled = use_signal(|| false);
    let mut search_text = use_signal(|| "".to_string());
    let items = props.items.read().clone();

    rsx! {
        div { margin: "10px", width: "100%", justify_content: "space-evenly",
            button {
                class: "bg-white w-full hover:bg-gray-100 text-gray-800 font-semibold border border-gray-400 rounded shadow",
                onclick: move |_| {
                    let t = toggled.read().clone();
                    toggled.set(!t);
                },
                span {
                    match props.limit {
                        Some(limit) if limit > 1 => {
                            rsx!( "select up to {limit} options" )
                        }
                        Some(_) => {
                            rsx!( "select an option" )
                        }
                        _ => {
                            rsx!( "select options" )
                        }
                    }
                    Icon {
                        class: "display: inline",
                        icon: FaSquareCaretDown,
                        width: 20,
                        height: 20,
                    }
                }
            }
            if toggled.read().clone() {
                div {
                    class: "w-full mt-1 rounded-md bg-white shadow-lg",
                    input {
                        class: "block w-full px-4 py-2 text-gray-800 border rounded-md  border-gray-300 focus:outline-none",
                        r#type: "text",
                        placeholder: "Search items",
                        autocomplete: "off",
                        oninput: move |e| {
                            search_text.set(e.data.value().clone());
                        }
                    }
                    ul {
                        class: "overflow-auto max-h-60 rounded-md py-1 text-base ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm",
                        for item in items {
                            if item.to_lowercase().contains(&search_text.read().to_lowercase()) {
                                li {
                                    class: "text-gray-900 cursor-default select-none relative py-2 pl-3 pr-9",
                                    div {
                                        class: "flex items-center",
                                        input {
                                            class: "absolute cursor-pointer",
                                            r#type: "checkbox",
                                            disabled: if let Some(limit) = props.limit { props.selected_options.read().len() >= limit && !props.selected_options.read().contains(&item.clone()) } else { false },
                                            id: "{item}",
                                            name: "{item}",
                                            value: "{item}",
                                            checked: "{props.selected_options.read().contains(&item.clone())}",
                                            onclick: move |_| {
                                                let mut selected = props.selected_options.read().clone();
                                                if selected.contains(&item.clone()) {
                                                    selected.retain(|x| x != &item.clone());
                                                } else {
                                                    selected.push(item.clone());
                                                }
                                                props.selected_options.set(selected);
                                            }
                                        }
                                        label {
                                            class: "ml-3 block font-normal truncate",
                                            r#for: "item", "{item}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct PokemonListProps {
    name: ReadOnlySignal<String>,
    selected_moves: ReadOnlySignal<Vec<String>>,
    selected_abilities: ReadOnlySignal<Vec<String>>,
    selected_types: ReadOnlySignal<Vec<String>>,
}

fn PokemonList(props: PokemonListProps) -> Element {
    let resp = use_resource(move || async move {
        let types = props.selected_types.read().clone().join("|");
        let request_body = Finder::build_query(finder::Variables {
            name: props.name.read().clone(),
            move_name: props.selected_moves.read().clone().join("|"),
            ability_name: props.selected_abilities.read().clone().join("|"),
            type_one: types.clone(),
            type_two: types.clone(),
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
            .await
            .expect("failed to parse response");

        resp.data.expect("missing response data")
    });

    match &*resp.read_unchecked() {
        Some(resp) => {
            rsx!(
                div { display: "flex", flex_direction: "row", flex_wrap: "wrap",
                    for pokemon in resp.pokemon_v2_pokemon.iter() {
                        Pokemon { pokemon: pokemon.clone() }
                    }
                }
            )
        }
        _ => rsx! {"Loading items"},
    }
}

#[derive(Props, PartialEq, Clone)]
struct PokemonProps {
    pokemon: finder::FinderPokemonV2Pokemon,
}

#[component]
fn Pokemon(props: PokemonProps) -> Element {
    let serebii_url = format!(
        "https://www.serebii.net/pokedex-sv/{}/",
        props.pokemon.name.clone()
    );
    let types = props
        .pokemon
        .pokemon_v2_pokemontypes
        .iter()
        .map(|t| {
            t.pokemon_v2_type
                .clone()
                .unwrap_or_default()
                .name
                .clone()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join(" / ");

    let abilities = props
        .pokemon
        .pokemon_v2_pokemonabilities
        .iter()
        .map(|a| {
            if a.is_hidden {
                format!(
                    "{} (hidden)",
                    a.pokemon_v2_ability
                        .clone()
                        .unwrap_or_default()
                        .name
                        .clone()
                        .to_string()
                )
            } else {
                a.pokemon_v2_ability
                    .clone()
                    .unwrap_or_default()
                    .name
                    .clone()
                    .to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" / ");

    rsx! {
        div { margin: "10px", width: "250px", height: "250px", border: "1px solid black",
            position: "relative",
            class: "group",
            div {
                class: "absolute bottom-0 w-full text-xl bg-sky-500 visible group-hover:hidden opacity-75 hover:opacity-100 place-content-center text-white",
                "{props.pokemon.name.clone()}"
            }
            div {
                class: "absolute top-0 w-full text-2xl bg-sky-500 invisible group-hover:visible opacity-75 place-content-center text-white",
                a {
                    href: "{serebii_url}",
                    target: "_blank",
                    "{props.pokemon.name.clone()}"
                }
                p { "{types}" }
                p { "{abilities}" }
            }
            img {
                src: "{props.pokemon.pokemon_v2_pokemonsprites[0]
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
        }
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
