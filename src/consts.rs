use phf::phf_map;

pub static BASE_API_URL: &str = "https://pokeapi.co/api/v2/";

pub static DEXES: [&str; 2] = ["paldea", "kitakami"];
pub static TYPES_INGREDIENTS: phf::Map<&'static str, &'static str> = phf_map! {
    "normal" => "tofu",
    "grass" => "lettuce",
    "fire" => "red pepper",
    "water" => "cucumber",
    "electric" => "yellow pepper",
    "ice" => "klawf stick",
    "fighting" => "pickle",
    "poison" => "green pepper",
    "ground" => "ham",
    "flying" => "prosciutto",
    "psychic" => "onion",
    "bug" => "cherry tomato",
    "rock" => "bacon",
    "ghost" => "red onion",
    "dragon" => "avocado",
    "dark" => "smoked fillet",
    "steel" => "hamburger",
    "fairy" => "tomato",
};
