use dioxus::prelude::*;
use dioxus_sdk::storage::use_persistent;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::footer;
use crate::BASE_REST_API_URL;

#[component]
pub fn Favourites() -> Element {
    let faves = use_persistent("faves", || HashSet::<String>::new());

    rsx! {
        h1 { "Favourites" }
        for fav in faves() {
            FavouritePokemon { pokemon_name: fav }
        }
        footer::Footer {}
    }
}

#[component]
fn FavouritePokemon(pokemon_name: ReadOnlySignal<String>) -> Element {
    let pokemon_res =
        use_resource(move || async move { get_pokemon(pokemon_name()).await.unwrap() });

    match &*pokemon_res.read_unchecked() {
        Some(pokemon) => rsx! { RenderPokemon { pokemon: pokemon.clone() } },
        _ => rsx! {"Loading items"},
    }
}

#[component]
fn RenderPokemon(pokemon: Pokemon) -> Element {
    let pokemon_name = pokemon.name.clone();
    let default_image = pokemon.sprites.other.official_artwork.front_default.clone();
    let shiny_image = pokemon.sprites.other.official_artwork.front_shiny.clone();

    rsx! {
        h1 { "{pokemon_name}" }
        div { display: "flex", flex_direction: "row",
            img { src: "{default_image}", width: "100%" }
            img { src: "{shiny_image.clone().unwrap_or_default()}", width: "100%" }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pokemon {
    name: String,
    sprites: Sprite,
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
pub struct OfficialArtwork {
    front_default: String,
    front_shiny: Option<String>,
}

async fn get_pokemon(name: String) -> Result<Pokemon, reqwest::Error> {
    log::info!("Fetching pokemon {}", name);
    let url = format!("{}/pokemon/{}", BASE_REST_API_URL, name);
    reqwest::get(&url).await?.json().await
}
