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
const HEADER_SVG: Asset = asset!("/assets/header.svg");
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
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    "💫 VSCode Extension"
                }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "container mx-auto px-4",
            h1 { class: "text-4xl font-bold mb-8 text-center",
                "Bienvenido a nuestra Tienda de Ropa Elegante"
            }
            FeaturedProducts {}
        }
    }
}

#[component]
fn Colecciones() -> Element {
    rsx! {
        div { class: "container mx-auto px-4",
            h2 { class: "text-3xl font-semibold mb-6", "Nuestras Colecciones" }
            ProductGrid {}
        }
    }
}

#[component]
fn Contacto() -> Element {
    rsx! {
        div { class: "container mx-auto px-4 py-8",
            h2 { class: "text-3xl font-semibold mb-6", "Contáctanos" }
            ContactForm {}
        }
    }
}

#[component]
fn Carrito() -> Element {
    rsx! {
        div { class: "container mx-auto px-4",
            h2 { class: "text-3xl font-semibold mb-6", "Tu Carrito" }
            ShoppingCart {}
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

// Placeholder components that we'll implement next
#[component]
fn FeaturedProducts() -> Element {
    rsx! {
        div { class: "grid grid-cols-1 md:grid-cols-3 gap-6", "Featured Products Coming Soon" }
    }
}

#[component]
fn ProductGrid() -> Element {
    rsx! {
        div { class: "grid grid-cols-1 md:grid-cols-4 gap-6", "Product Grid Coming Soon" }
    }
}

#[component]
fn ContactForm() -> Element {
    rsx! {
        div { class: "max-w-lg mx-auto", "Contact Form Coming Soon" }
    }
}

#[component]
fn ShoppingCart() -> Element {
    rsx! {
        div { class: "max-w-2xl mx-auto", "Shopping Cart Coming Soon" }
    }
}
