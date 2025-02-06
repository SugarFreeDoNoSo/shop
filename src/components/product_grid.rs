use dioxus::prelude::*;


#[component]
pub fn ProductGrid() -> Element {
    rsx! {
        div { class: "grid grid-cols-1 md:grid-cols-4 gap-6", "Product Grid Coming Soon" }
    }
}



