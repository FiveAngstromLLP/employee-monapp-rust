mod backend;
mod frontend;

use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <frontend::EmployeeApp />
    }
}

#[tokio::main]
async fn main() {
    leptos::start_app(App);
    backend::rocket().launch().await.unwrap();
}