use phf::phf_map;

mod dex_by_type;
mod favourites;
mod focus;
mod shiny_dex;

struct TypeInfo {
    pub color: &'static str,
    pub ingredient: &'static str,
}

static TYPES_INFO: phf::Map<&'static str, TypeInfo> = phf_map! {
    "normal" => TypeInfo {
        color: "#A8A77A",
        ingredient: "tofu",
    },
    "grass" => TypeInfo {
        color: "#7AC74C",
        ingredient: "lettuce",
    },
    "fire" => TypeInfo {
        color: "#EE8130",
        ingredient: "red pepper",
    },
    "water" => TypeInfo {
        color: "#6390F0",
        ingredient: "cucumber",
    },
    "electric" => TypeInfo {
        color: "#F7D02C",
        ingredient: "yellow pepper",
    },
    "ice" => TypeInfo {
        color: "#96D9D6",
        ingredient: "klawf stick",
    },
    "fighting" => TypeInfo {
        color: "#C22E28",
        ingredient: "pickle",
    },
    "poison" => TypeInfo {
        color: "#A33EA1",
        ingredient: "green pepper",
    },
    "ground" => TypeInfo {
        color: "#E2BF65",
        ingredient: "ham",
    },
    "flying" => TypeInfo {
        color: "#A98FF3",
        ingredient: "prosciutto",
    },
    "psychic" => TypeInfo {
        color: "#F95587",
        ingredient: "onion",
    },
    "bug" => TypeInfo {
        color: "#A6B91A",
        ingredient: "cherry tomato",
    },
    "rock" => TypeInfo {
        color: "#B6A136",
        ingredient: "bacon",
    },
    "ghost" => TypeInfo {
        color: "#735797",
        ingredient: "red onion",
    },
    "dragon" => TypeInfo {
        color: "#6F35FC",
        ingredient: "avocado",
    },
    "dark" => TypeInfo {
        color: "#705746",
        ingredient: "smoked fillet",
    },
    "steel" => TypeInfo {
        color: "#B7B7CE",
        ingredient: "hamburger",
    },
    "fairy" => TypeInfo {
        color: "#D685AD",
        ingredient: "tomato",
    },
};

pub mod prelude {
    pub use crate::shiny_dex::favourites::Favourites;
    pub use crate::shiny_dex::shiny_dex::ShinyDex;
}
