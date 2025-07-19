use dioxus::prelude::*;

#[component]
pub fn BlogPreview(id: String) -> Element {
    let div_id = format!("blog-preview-{}", id);
    let preview_html = use_server_future(move || api::blog_preview(id.clone()))?()
        .unwrap()
        .unwrap_or_default();

    rsx! {
        div {
            class: "{div_id}",
            dangerous_inner_html: "{preview_html}",
        },
    }
}
