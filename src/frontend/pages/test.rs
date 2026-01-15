use dioxus::prelude::*;

use crate::backend::api::{api_manufacturer::list_manu, service::text_stream2};

#[component]
pub fn Test() -> Element {
    let mut stream_output: Signal<Vec<String>> = use_signal(|| vec![]);
    use_future(move || async move {
        if let Ok(mut stream) = text_stream2().await {
            while let Some(Ok(text)) = stream.next().await {
                //stream_output.write().push_str(&text);
                //stream_output.write().clone_from(&text);
                stream_output.write().push(text);
            }
        }
    });

    rsx! {
        p { "stream" }
        for i in stream_output.read().iter() {
            span { "{i}" }
            hr {}
        }
        //p { "{stream_output}" }
    }

    /*let mut breed = use_signal(|| "hound".to_string());
    let dogs = use_resource(move || async move { list_manu });

    rsx! {
        input {
            value: "{breed}",
            // When the input is changed and the breed is set, the resource will rerun
            oninput: move |evt| breed.set(evt.value()),
        }

        div {
            display: "flex",
            flex_direction: "row",
            // You can read resource just like a signal. If the resource is still
            // running, it will return None
            if let Some(response) = &*dogs.read() {
                "got"
            } else {
                "Loading..."
            }
        }
    }*/
}
