#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::LevelFilter;

mod consts;
mod dex_by_type;
mod favourites;
mod footer;
mod home;

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/dex/:dex/type/:pokemon_type")]
        DexByType {
            dex: String,
            pokemon_type: String,
        },
        #[route("/favourites")]
        Favourites {},
    #[end_layout]
    #[route("/:.._route")]
    PageNotFound {
        _route: Vec<String>,
    },
}

pub fn App(cx: Scope) -> Element {
    render! { Router::<Route> {} }
}

pub fn Home(cx: Scope) -> Element {
    render! { home::Home {} }
}

#[inline_props]
pub fn DexByType(cx: Scope, dex: String, pokemon_type: String) -> Element {
    render! { dex_by_type::DexByType { dex: dex.clone(), pokemon_type: pokemon_type.clone() } }
}

#[inline_props]
pub fn Favourites(cx: Scope) -> Element {
    render! { favourites::Favourites {} }
}

#[inline_props]
fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            display: "flex",
            flex_direction: "row",
            justify_content: "space-between",
            align_items: "center",
            background_color: "grey",
            color: "white",
            padding: "10px",
            Link { to: "/", "Pokemon Dex" }
            Link { to: "/favourites", "Favourites" }
        }
        Outlet::<Route> {}
    }
}

#[inline_props]
fn PageNotFound(cx: Scope, _route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
    }
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(App);
}
