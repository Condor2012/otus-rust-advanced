use dioxus::prelude::*;

#[component]
pub fn Test(name: String) -> Element {
    rsx! {
        div { "Name: {name}" }
    }
}
