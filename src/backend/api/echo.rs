use dioxus::prelude::*;

#[post("/api/echo")]
pub async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
