pub mod rgb_picker;
use crate::rgb_picker::RgbPicker;

use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/>})
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <RgbPicker/>
        </div>
    }
}
