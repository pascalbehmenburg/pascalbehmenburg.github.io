use dioxus::prelude::*;

use crate::BlogPreview;

/// Display blog content as html.
#[component]
pub fn Feed() -> Element {
    let blog_ids = use_server_future(api::get_blog_ids)?()
        .unwrap()
        .unwrap_or_default();

    rsx! {
        div {
            {blog_ids.iter().map(|id| {
                rsx! {
                    BlogPreview { id: id },
                    hr { width: "100%", size: "10"  }
                }
            })}
        }
    }
}
