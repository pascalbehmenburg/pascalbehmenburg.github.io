use dioxus::prelude::*;

/// Display blog content as html.
#[component]
pub fn Blog(id: String) -> Element {
    let div_id = format!("blog-{}", id);
    let html = use_server_future(move || api::blog(String::from(&id)))?()
        .unwrap()
        .unwrap_or_default();

    rsx! {
        div {
            id: "{div_id}",
            dangerous_inner_html: "{html}"
        }
    }
}
