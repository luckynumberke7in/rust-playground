pub mod todo;
use crate::todo::Todo;

use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/>})
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <Todo/>
        </div>
    }
}
