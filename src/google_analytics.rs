use dioxus::prelude::*;

#[component]
pub fn GoogleAnalytics(config: String) -> Element {
    rsx! {
        script {
            src: "https://www.googletagmanager.com/gtag/js?id={config}",
        }
        script {
            dangerous_inner_html: r#"
                window.dataLayer = window.dataLayer || [];
                function gtag(){{dataLayer.push(arguments);}}
                gtag('js', new Date());
                gtag('config', '{config}');"#,
        }
    }
}
