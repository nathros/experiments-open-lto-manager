use dioxus::prelude::*;

use crate::backend::database::db::{list_tapes, save_tape, Tape};

/// Home page
#[component]
pub fn Home() -> Element {
    let mut tapes_list = use_loader(list_tapes)?;
    let mut bar = use_signal(|| "".to_string());

    rsx! {
        p { "list" }
        input {
            oninput: move |e| bar.set(e.value()),
        }
        button {
            id: "save",
            onclick: move |_| async move { _ = save_tape(Tape {
                id: 0,
                barcode: bar.to_string(),
                worm: false
            }).await;
            tapes_list.restart();
         },
            "save!"
        }
        br {  }
        br {  }
        table {
            tr {
                th { "id" }
                th { "barcode" }
                th { "worm" }
            }
            for (id , t) in tapes_list.cloned() {
                tr {
                    th { "{id}" }
                    th { "{t.barcode}" }
                    th { "{t.worm}" }
                }
            }
        }
    }
}
