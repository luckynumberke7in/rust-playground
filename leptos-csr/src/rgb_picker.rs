use leptos::*;

#[component]
pub fn RgbPicker(cx: Scope) -> impl IntoView {
    let (count_r, set_count_r) = create_signal(cx, 10);
    let (count_g, set_count_g) = create_signal(cx, 10);
    let (count_b, set_count_b) = create_signal(cx, 10);
    let (input_r, set_input_r) = create_signal(cx, "".to_string());
    let (input_g, set_input_g) = create_signal(cx, "".to_string());
    let (input_b, set_input_b) = create_signal(cx, "".to_string());

    view! { cx,
        <div
            style:background-color=move || format!("rgb({}, {}, {})", count_r(), count_g(), count_b())
        >
        <div>
            <p>"This is the blended values of Red Green and Blue"</p>
        </div>
        <div
            style:background-color=move || format!("rgb({}, 0, 0", count_r())
        >
            <button // - R
                on:click=move |_| {
                    set_count_r.update(|n| *n-=1);
                }
            >
                "Less Red"
            </button>
            {count_r}
            <button // + R
                on:click=move |_| {
                    set_count_r.update(|n| *n+=1);
                }
            >
                "More Red"
            </button>
            <progress
                max="255"
                value=count_r
            />
            <div>
                <input type="text"
                    on:input=move |ev| {
                        set_input_r(event_target_value(&ev));
                    }
                >
                </input>
                <button
                    on:click=move |_| {
                        set_count_r(input_r.get_untracked().parse::<i32>().ok().unwrap_or_default())
                    }
                >
                    "Set Red"
                </button>
            </div>
        </div>
        <div
            style:background-color=move || format!("rgb(0, {}, 0", count_g())
        >
            <button // G
                on:click=move |_| {
                    set_count_g.update(|n| *n-=1);
                }
            >
                "Less Green"
            </button>
            {count_g}
            <button // G
                on:click=move |_| {
                    set_count_g.update(|n| *n+=1);
                }
            >
                "More Green"
            </button>
            <progress
                max="255"
                value=count_g
            />
            <div>
                <input type="text"
                    on:input=move |ev| {
                        set_input_g(event_target_value(&ev));
                    }
                >
                </input>
                <button
                    on:click=move |_| {
                        set_count_g(input_g.get_untracked().parse::<i32>().ok().unwrap_or_default())
                    }
                >
                    "Set Green"
                </button>
            </div>
        </div>
        <div
            style:background-color=move || format!("rgb(0, 0, {}", count_b())
        >
            <button // B
                on:click=move |_| {
                    set_count_b.update(|n| *n-=1);
                }
            >
                "Less Blue"
            </button>
            //{move || count().get} // on stable version, this would be count.get() instead.
            {count_b} // this is already a function in nightly Leptos, the closure above is redundant.
                // count is already a reactive fn, so doing count() here would unwrap the reactivity.
            <button // B
                on:click=move |_| {
                    set_count_b.update(|n| *n+=1);
                }
            >
                "More Blue"
            </button>
            <progress
                max="255"
                value=count_b
            />
            <div>
                <input type="text"
                    on:input=move |ev| {
                        set_input_b(event_target_value(&ev));
                    }
                >
                </input>
                <button
                    on:click=move |_| {
                        set_count_b(input_b.get_untracked().parse::<i32>().ok().unwrap_or_default())
                    }
                >
                "Set Blue"
                </button>
            </div>
        </div>
        </div>
    }
}
