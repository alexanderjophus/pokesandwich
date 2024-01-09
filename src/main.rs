#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::LevelFilter;

mod consts;
mod dex_by_type;
mod favourites;
use favourites::Favourites;
mod footer;
use footer::Footer;
mod home;
use home::Home;

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
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

#[component]
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
            Link { to: "/", "Pokédex" }
            Link { to: "/favourites", "Favourites" }
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
