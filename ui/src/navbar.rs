use dioxus::prelude::*;

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        table {
            tr {
                {children}
            }
        }
    }
}
