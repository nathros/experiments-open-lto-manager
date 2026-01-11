use dioxus::prelude::*;

use crate::frontend::pages::home::Home;
use crate::frontend::components::navbar::Navbar;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},

    //#[route("/blog/:id")]
    //Blog { id: i32 },
}