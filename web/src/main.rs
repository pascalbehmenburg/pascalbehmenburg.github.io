use dioxus::{document::Script, prelude::*};
use ui::Navbar;
use views::{Blog, Feed};

mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HIGHLIGHT_JS: Asset = asset!("/assets/highlight.min.js");
const ROSE_PINE_DAWN: Asset = asset!("/assets/ros-pine-dawn.min.css");
const ROSE_PINE_MOON: Asset = asset!("/assets/ros-pine-moon.min.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Feed {},
    #[route("/blog/:id")]
    Blog { id: String },
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    rsx! {
        Navbar {
            td {
                Link {
                    to: Route::Feed {},
                    "Feed"
                }
            }
        }

        Outlet::<Route> {}
    }
}

#[component]
fn App() -> Element {
    let rose_pin_css = format!(
        r#"
        @import url("{}") screen;
        @import url("{}") screen and (prefers-color-scheme: dark);"#,
        ROSE_PINE_DAWN, ROSE_PINE_MOON
    );

    rsx! {
        // global app stylesheets
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Script { type: "text/javascript", src: HIGHLIGHT_JS }
        Script { type: "text/javascript", "hljs.highlightAll();" }
        div {
            dangerous_inner_html: "<style type=\"text/css\">{rose_pin_css}</style>"
        }
        main {
            Router::<Route> {}
        }
    }
}

fn main() {
    dioxus::launch(App);
}
