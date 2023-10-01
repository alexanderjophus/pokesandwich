use clap::Parser;
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Serialize, Deserialize)]
struct Pokedex {
    pokemon_entries: Vec<PokemonEntry>,
}

#[derive(Serialize, Deserialize)]
struct PokemonEntry {
    pokemon_species: PokemonSpecies,
}

#[derive(Serialize, Deserialize)]
struct PokemonSpecies {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Type {
    pokemon: Vec<Pokemon>,
}

#[derive(Serialize, Deserialize)]
struct Pokemon {
    pokemon: PokemonReference,
}

#[derive(Serialize, Deserialize)]
struct PokemonReference {
    name: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pokemon_type: String,

    #[arg(short, long, default_value = "paldea")]
    dex: String,
}

fn main() {
    let args = Args::parse();

    println!("Looking for {}s in {}!", args.pokemon_type, args.dex);

    let mut pokedex_res =
        reqwest::blocking::get(format!("https://pokeapi.co/api/v2/pokedex/{}", args.dex))
            .expect("Failed to send request");
    let mut body = String::new();
    pokedex_res
        .read_to_string(&mut body)
        .expect("Failed to read response body");

    let pokedex: Pokedex = serde_json::from_str(&body).unwrap();

    let mut type_res = reqwest::blocking::get(format!(
        "https://pokeapi.co/api/v2/type/{}",
        args.pokemon_type
    ))
    .expect("Failed to send request");
    let mut body = String::new();
    type_res
        .read_to_string(&mut body)
        .expect("Failed to read response body");

    let pokemon_type: Type = serde_json::from_str(&body).unwrap();

    let pokemon_names: Vec<String> = pokemon_type
        .pokemon
        .iter()
        .map(|pokemon| pokemon.pokemon.name.clone())
        .collect();

    let pokemon_entries: Vec<PokemonEntry> = pokedex
        .pokemon_entries
        .into_iter()
        .filter(|entry| pokemon_names.contains(&entry.pokemon_species.name))
        .collect();

    let pokemon_names: Vec<String> = pokemon_entries
        .into_iter()
        .map(|entry| entry.pokemon_species.name)
        .collect();

    println!("Found {:?}!", pokemon_names);
}
