#![allow(non_snake_case)]

mod layouts;

use dioxus::desktop::Config;
use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}


fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    let cfg: Config = Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string())
        .with_custom_index(include_str!("static/index.html").to_string())
        .with_background_color((23, 36, 84, 1));

    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}


#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
            div {
                    p {
                        "Hello world"
                    }
                }
    }
}

