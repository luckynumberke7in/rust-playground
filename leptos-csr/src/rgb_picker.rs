use leptos::*;

#[component]
pub fn RgbPicker(cx: Scope) -> impl IntoView {
    let (value_r, set_value_r) = create_signal(cx, 10);
    let (value_g, set_value_g) = create_signal(cx, 10);
    let (value_b, set_value_b) = create_signal(cx, 10);

    view! { cx,
        <div>
            <div
                style:background-color=move || format!("rgb({}, {}, {})", value_r(), value_g(), value_b())
            >
                <div>
                    <p>"This is the blended values of Red Green and Blue"</p>
                </div>
                <div>
                    <p>"..."</p>
                </div>
            </div>
            <div
                style:background-color=move || format!("rgb({}, 0, 0", value_r())
            >
                <RgbInput
                    color="Red".to_string()
                    value=value_r
                    set_value=set_value_r
                />
            </div>
            <div
                style:background-color=move || format!("rgb(0, {}, 0", value_g())
            >
                <RgbInput
                    color="Green".to_string()
                    value=value_g
                    set_value=set_value_g
                />
            </div>
            <div
                style:background-color=move || format!("rgb(0, 0, {}", value_b())
            >
                <RgbInput
                    color="Blue".to_string()
                    value=value_b
                    set_value=set_value_b
                />
            </div>
        </div>
    }
}

#[component]
fn RgbInput(
    cx: Scope,
    color: String,
    value: ReadSignal<i32>,
    set_value: WriteSignal<i32>,
) -> impl IntoView {
    let (input, set_input) = create_signal(cx, "".to_string());

    view! {cx,
        <div>
            <button // B
                on:click=move |_| {
                    set_value.update(|n| *n-=1);
                }
            >
                "Less " {&color}
            </button>
            //{move || value().get} // on stable version, this would be value.get() instead.
            {value} // this is already a function in nightly Leptos, the closure above is redundant.
                // value is already a reactive fn, so doing value() here would unwrap the reactivity.
            <button // B
                on:click=move |_| {
                    set_value.update(|n| *n+=1);
                }
            >
                "More " {&color}
            </button>
            <progress
                max="255"
                value=value
            />
            <div>
                <input type="text"
                    on:input=move |ev| {
                        set_input(event_target_value(&ev));
                    }
                >
                </input>
                <button
                    on:click=move |_| {
                        set_value(input.get_untracked().parse::<i32>().ok().unwrap_or_default())
                    }
                >
                    "Set " {&color}
                </button>
            </div>
        </div>
    }
}
