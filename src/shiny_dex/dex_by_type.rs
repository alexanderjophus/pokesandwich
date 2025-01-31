use std::collections::HashSet;

use crate::shiny_dex::focus::{dex_by_type, load_focus, perform_gql_query, Focus, FocusState};
use crate::shiny_dex::TYPES_INFO;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use dioxus_sdk::storage::use_persistent;

#[component]
pub fn DexByType(dex: Signal<String>, pokemon_type: Signal<String>) -> Element {
    let focus_state = use_signal(|| FocusState::Unset);

    rsx! {
        div { display: "flex", flex_direction: "row",
            div { overflow: "auto", max_height: "100vh", margin: "10px", width: "20%",
                Search { focus_state: focus_state, dex: dex.clone(), pokemon_type: pokemon_type.clone() }
            }
            div { margin: "10px", width: "80%", Focus { focus_state } }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct SearchProps {
    focus_state: Signal<FocusState>,
    dex: Signal<String>,
    pokemon_type: Signal<String>,
}

fn Search(props: SearchProps) -> Element {
    let faves = use_persistent("faves", || HashSet::<String>::new());
    let pokemon = use_resource(move || async move {
        let variables = dex_by_type::Variables {
            dex: props.dex.to_string().clone(),
            pokemon_type: props.pokemon_type.to_string().clone(),
        };
        perform_gql_query(variables).await
    });

    match &*pokemon.read_unchecked() {
        Some(Ok(pokemon)) => {
            rsx! { RenderDex { faves: faves,  focus_state: props.focus_state, pokemon: pokemon.clone(), pokemon_type: props.pokemon_type.clone() } }
        }
        Some(Err(err)) => rsx! {"An error occurred while loading {err}"},
        _ => rsx! {"Loading items"},
    }
}

#[derive(PartialEq, Props, Clone)]
struct RenderDexProps {
    faves: Signal<HashSet<String>>,
    focus_state: Signal<FocusState>,
    pokemon: ReadOnlySignal<Vec<dex_by_type::DexByTypePokemonV2Pokemon>>,
    pokemon_type: String,
}

#[component]
fn RenderDex(props: RenderDexProps) -> Element {
    rsx! {
        div { overflow: "hidden", background_color: TYPES_INFO.get(props.pokemon_type.as_str()).unwrap().color, border_radius: "50%", width: "100px", height: "100px", img { src: "/icons/{props.pokemon_type.clone()}.svg" } }
        div { overflow: "auto", display: "flex", flex_direction: "column", width: "100%", DexTable { faves: props.faves,  focus_state: props.focus_state, pokemon: props.pokemon } }
    }
}

#[component]
fn DexTable(
    faves: Signal<HashSet<String>>,
    focus_state: Signal<FocusState>,
    pokemon: ReadOnlySignal<Vec<dex_by_type::DexByTypePokemonV2Pokemon>>,
) -> Element {
    rsx! {
        table { border_collapse: "collapse",
            thead {
                tr { th { "Pokemon Name" } }
            }
            tbody {
                for entry in pokemon() {
                    DexRow { faves: faves, focus_state, entry: entry.clone() }
                }
            }
        }
    }
}

#[component]
fn DexRow(
    faves: Signal<HashSet<String>>,
    focus_state: Signal<FocusState>,
    entry: ReadOnlySignal<dex_by_type::DexByTypePokemonV2Pokemon>,
) -> Element {
    rsx! {
        tr { class: "border-2 hover:bg-gray-100 hover:ring-2 hover:ring-pink-500 hover:ring-inset",
            td {
                div { display: "flex", flex_direction: "row",
                    div {
                        width: "80%",
                        onclick: move |_| { load_focus(focus_state, entry) },
                        "{entry().name}"
                    }
                    div {
                        width: "20%",
                        onclick: move |_|  {
                            let name = &entry().name;
                            info!("clicked on {name}");
                            if faves().contains(&entry().name) {
                                info!("removing {name} from faves");
                                faves().remove(&entry().name);
                            } else {
                                info!("adding {name} to faves");
                                faves().insert(entry().name);
                            }
                        },
                        i { class: "fa fa-heart", color: if faves().contains(&entry().name) { "red" } else { "grey" }},
                    }
                }
            }
        }
    }
}
