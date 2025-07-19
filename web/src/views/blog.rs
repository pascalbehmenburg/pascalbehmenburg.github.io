use dioxus::prelude::*;

#[component]
pub fn Blog(id: String) -> Element {
    rsx! {
        ui::Blog { id: String::from(&id) }
    }
}
