#![allow(non_snake_case)]
use crate::document::Script;
use crate::document::Stylesheet;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use dioxus_router::prelude::*;

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

#[derive(Clone, Routable)]
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

#[component]
fn NavBar() -> Element {
    rsx! {
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
            Link { to: "/finder", "Pokémon Finder" }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn PageNotFound(_route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        Footer {}
    }
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::LaunchBuilder::new().launch(|| {
        rsx! {
            Stylesheet { href: asset!("./public/tailwind.css") }
            Script { src: "https://cdn.jsdelivr.net/npm/echarts@5.5.1/dist/echarts.min.js" }
            Script { src: "https://cdn.jsdelivr.net/npm/echarts-gl@2.0.9/dist/echarts-gl.min.js" }
            google_analytics::GoogleAnalytics { config: "G-HJKW1YL1C9" }
            Router::<Route> {}
        }
    });
}
