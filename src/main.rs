#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::LevelFilter;

mod home;
use home::Home;
mod footer;
use footer::Footer;
mod shiny_dex;
use shiny_dex::prelude::*;
mod pokemon_finder;
use pokemon_finder::prelude::*;
mod google_analytics;

pub static BASE_GRAPHQL_API_URL: &str = "https://beta.pokeapi.co/graphql/v1beta";
pub static BASE_REST_API_URL: &str = "https://pokeapi.co/api/v2";

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[nest("/shiny")]
            #[route("/")]
            ShinyDex {},
            #[route("/favourites")]
            Favourites {},
        #[end_nest]
        #[nest("/finder")]
            #[route("/")]
            PokemonFinder {},
    #[end_layout]
    #[route("/:.._route")]
    PageNotFound {
        _route: Vec<String>,
    },
}

pub fn App(cx: Scope) -> Element {
    render! {
        google_analytics::GoogleAnalytics { config: "G-HJKW1YL1C9" }
        Router::<Route> {}
    }
}

#[component]
fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            display: "flex",
            flex_direction: "row",
            justify_content: "space-evenly",
            align_items: "center",
            background_color: "grey",
            color: "white",
            padding: "10px",
            Link { to: "/", "Home" }
            Link { to: "/shiny", "Shiny Dex" }
            Link { to: "/finder", "Pokémon Finder" span { color: "red", " (WIP)" } }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn PageNotFound(cx: Scope, _route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        Footer {}
    }
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(App);
}
