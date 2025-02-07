use dioxus::prelude::*;

#[component]
pub fn FeaturedProducts() -> Element {
    rsx! {
        div { class: "grid grid-cols-1 md:grid-cols-3 gap-6", "Featured Products Coming Soon" }
    }
}
