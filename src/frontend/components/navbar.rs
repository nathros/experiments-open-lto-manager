use dioxus::prelude::*;

use crate::Route;

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            br {  }
            Link { to: Route::Show {}, "Show" }
            br {  }
            Link { to: Route::Test {}, "Test" }
            //Link { to: Route::Blog { id: 1 }, "Blog" }
        }

        Outlet::<Route> {}
    }
}
