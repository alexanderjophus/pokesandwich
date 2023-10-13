use dioxus::prelude::*;

#[inline_props]
pub fn Footer(cx: Scope) -> Element {
    render! {
        script {
            src: "https://kit.fontawesome.com/e04bfc6d26.js",
            crossorigin: "anonymous",
        }
        footer {
            position: "fixed",
            bottom: "0",
            left: "0",
            right: "0",
            height: "50px",
            background_color: "grey",
            color: "white",
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            "Made with "
            span {
                color: "red",
                "❤️"
            }
            " by Alexander Jophus"
            a {
                href: "https://github.com/alexanderjophus",
                target: "_blank",
                i {
                    class: "fa fa-github",
                    font_size: "30px",
                    margin_left: "10px",
                    color: "white",
                }
            }
            a {
                href: "https://twitter.com/alexanderjophus",
                target: "_blank",
                i {
                    class: "fa fa-twitter",
                    font_size: "30px",
                    margin_left: "10px",
                    margin_right: "10px",
                    color: "white",
                }
            }
            "Pokemon data from "
            a {
                href: "https://pokeapi.co/",
                target: "_blank",
                "pokeapi.co"
            }
        }
    }
}
