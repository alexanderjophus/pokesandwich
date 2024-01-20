use dioxus::prelude::*;
use dioxus_storage::use_persistent;
use std::collections::HashSet;

use crate::shiny_dex::focus::{dex_by_type, load_focus, perform_gql_query, Focus, FocusState};
use crate::shiny_dex::TYPES_INFO;

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
        let variables = dex_by_type::Variables {
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
    pokemon: Vec<dex_by_type::DexByTypePokemonV2Pokemon>,
) -> Element {
    cx.render(rsx! {
        div { overflow: "hidden", background_color: TYPES_INFO.get(cx.props.pokemon_type.as_str()).unwrap().color, border_radius: "50%", width: "100px", height: "100px", img { src: "/icons/{cx.props.pokemon_type.clone()}.svg" } }
        div { overflow: "auto", display: "flex", flex_direction: "column", width: "100%", DexTable { pokemon: pokemon } }
    })
}

#[component]
fn DexTable(cx: Scope, pokemon: Vec<dex_by_type::DexByTypePokemonV2Pokemon>) -> Element {
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
fn DexRow(cx: Scope, entry: dex_by_type::DexByTypePokemonV2Pokemon) -> Element {
    let fav = use_persistent(cx, "faves", || HashSet::new());
    let focus_state = use_shared_state::<FocusState>(cx).unwrap();

    cx.render(rsx! {
        tr { class: "border-2 hover:bg-gray-100 hover:ring-2 hover:ring-pink-500 hover:ring-inset",
            td {
                div { display: "flex", flex_direction: "row",
                    div {
                        width: "80%",
                        onclick: move |_event| { load_focus(focus_state.clone(), entry.clone()) },
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
