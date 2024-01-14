use dioxus::prelude::*;

#[derive(Props)]
pub struct GoogleAnalyticsProps<'a> {
    config: &'a str,
}

pub fn GoogleAnalytics<'a>(cx: Scope<'a, GoogleAnalyticsProps>) -> Element<'a> {
    cx.render(rsx! {
        script {
            src: "https://www.googletagmanager.com/gtag/js?id={cx.props.config}",
        }
        script {
            dangerous_inner_html: r#"
                window.dataLayer = window.dataLayer || [];
                function gtag(){{dataLayer.push(arguments);}}
                gtag('js', new Date());
                gtag('config', '{cx.props.config}');"#,
        }
    })
}
