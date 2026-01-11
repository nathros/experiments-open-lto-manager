use dioxus::prelude::*;

use crate::frontend::components::navbar::Navbar;
use crate::frontend::pages::home::Home;
use crate::frontend::pages::show::Show;
use crate::frontend::pages::test::Test;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},

    #[route("/show")]
    Show {},

    #[route("/test")]
    Test {},

    //#[route("/blog/:id")]
    //Blog { id: i32 },
}
