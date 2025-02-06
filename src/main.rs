mod components;

// use components;

use dioxus::prelude::*;

// use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/colecciones")]
    Colecciones {},
    #[route("/contacto")]
    Contacto {},
    #[route("/carrito")]
    Carrito {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/output.css");

fn main() {
    dioxus::launch(App);
}


#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}


#[component]
fn Home() -> Element {
    rsx! {
        div { class: "container mx-auto px-4",
            h1 { class: "text-4xl font-bold mb-8 text-center",
                "Bienvenido a nuestra Tienda de Ropa Elegante"
            }
            components::featured_products::FeaturedProducts {}
        }
    }
}

#[component]
fn Colecciones() -> Element {
    rsx! {
        div { class: "container mx-auto px-4",
            h2 { class: "text-3xl font-semibold mb-6", "Nuestras Colecciones" }
            components::product_grid::ProductGrid {}
        }
    }
}

#[component]
fn Contacto() -> Element {
    rsx! {
        div { class: "container mx-auto px-4 py-8",
            h2 { class: "text-3xl font-semibold mb-6", "Contáctanos" }
            components::contact_form::ContactForm {}
        }
    }
}

#[component]
fn Carrito() -> Element {
    rsx! {
        div { class: "container mx-auto px-4",
            h2 { class: "text-3xl font-semibold mb-6", "Tu Carrito" }
            components::shopping_cart::ShoppingCart {}
        }
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        nav { class: "bg-white shadow-lg",
            div { class: "container mx-auto px-4",
                div { class: "flex justify-between items-center h-16",
                    // Logo
                    div { class: "flex-shrink-0",
                        Link {
                            to: Route::Home {},
                            class: "text-xl font-semibold text-gray-800",
                            "Elegancia"
                        }
                    }
                    // Navigation Links
                    div { class: "hidden md:block",
                        div { class: "ml-10 flex items-baseline space-x-4",
                            Link {
                                to: Route::Home {},
                                class: "text-gray-800 hover:text-gray-600 px-3 py-2 rounded-md text-sm font-medium",
                                "Inicio"
                            }
                            Link {
                                to: Route::Colecciones {},
                                class: "text-gray-800 hover:text-gray-600 px-3 py-2 rounded-md text-sm font-medium",
                                "Colecciones"
                            }
                            Link {
                                to: Route::Contacto {},
                                class: "text-gray-800 hover:text-gray-600 px-3 py-2 rounded-md text-sm font-medium",
                                "Contacto"
                            }
                            Link {
                                to: Route::Carrito {},
                                class: "text-gray-800 hover:text-gray-600 px-3 py-2 rounded-md text-sm font-medium",
                                "Carrito"
                            }
                        }
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}

