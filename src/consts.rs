use phf::phf_map;

pub static BASE_API_URL: &str = "https://pokeapi.co/api/v2/";

pub static DEXES: [&str; 3] = ["paldea", "kitakami", "blueberry"];

pub static TYPES: [&str; 18] = [
    "normal", "grass", "fire", "water", "electric", "ice", "fighting", "poison", "ground",
    "flying", "psychic", "bug", "rock", "ghost", "dragon", "dark", "steel", "fairy",
];

pub struct TypeInfo {
    pub name: &'static str,
    pub color: &'static str,
    pub ingredient: &'static str,
}

pub static TYPES_INFO: phf::Map<&'static str, TypeInfo> = phf_map! {
    "normal" => TypeInfo {
        name: "normal",
        color: "#A8A77A",
        ingredient: "tofu",
    },
    "grass" => TypeInfo {
        name: "grass",
        color: "#7AC74C",
        ingredient: "lettuce",
    },
    "fire" => TypeInfo {
        name: "fire",
        color: "#EE8130",
        ingredient: "red pepper",
    },
    "water" => TypeInfo {
        name: "water",
        color: "#6390F0",
        ingredient: "cucumber",
    },
    "electric" => TypeInfo {
        name: "electric",
        color: "#F7D02C",
        ingredient: "yellow pepper",
    },
    "ice" => TypeInfo {
        name: "ice",
        color: "#96D9D6",
        ingredient: "klawf stick",
    },
    "fighting" => TypeInfo {
        name: "fighting",
        color: "#C22E28",
        ingredient: "pickle",
    },
    "poison" => TypeInfo {
        name: "poison",
        color: "#A33EA1",
        ingredient: "green pepper",
    },
    "ground" => TypeInfo {
        name: "ground",
        color: "#E2BF65",
        ingredient: "ham",
    },
    "flying" => TypeInfo {
        name: "flying",
        color: "#A98FF3",
        ingredient: "prosciutto",
    },
    "psychic" => TypeInfo {
        name: "psychic",
        color: "#F95587",
        ingredient: "onion",
    },
    "bug" => TypeInfo {
        name: "bug",
        color: "#A6B91A",
        ingredient: "cherry tomato",
    },
    "rock" => TypeInfo {
        name: "rock",
        color: "#B6A136",
        ingredient: "bacon",
    },
    "ghost" => TypeInfo {
        name: "ghost",
        color: "#735797",
        ingredient: "red onion",
    },
    "dragon" => TypeInfo {
        name: "dragon",
        color: "#6F35FC",
        ingredient: "avocado",
    },
    "dark" => TypeInfo {
        name: "dark",
        color: "#705746",
        ingredient: "smoked fillet",
    },
    "steel" => TypeInfo {
        name: "steel",
        color: "#B7B7CE",
        ingredient: "hamburger",
    },
    "fairy" => TypeInfo {
        name: "fairy",
        color: "#D685AD",
        ingredient: "tomato",
    },
};
