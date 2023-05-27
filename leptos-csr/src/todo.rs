use leptos::*;

#[component]
pub fn Todo(cx: Scope) -> impl IntoView {
    let static_todos = vec![
        "make normal list with .map --done",
        "make signal for todos (vec containing a string, bool, and id",
        "make todo input and add todo button",
        "make toggle todo, delete, & maybe edit",
        "make for component to iterate over them",
    ];
    struct TodoData {
        id: usize,
        title: String,
        is_complete: bool,
    }

    let (todos, set_todos) = create_signal(cx, Vec::new());

    // change methods for updating id. this is insufficient and will introduce bugs.
    let curr_id = move || todos().len() + 1;
    let (input, set_input) = create_signal(cx, "".to_string());

    view! {cx,
        <div>
            <ul>
                {static_todos.into_iter()
                    .map(|n| view! { cx, <li>{n}</li> })
                    .collect::<Vec<_>>()}
            </ul>
            <div> // add todo's
                <input type="text"
                on:input=move |ev| {
                    set_input(event_target_value(&ev));
                }
                >
                </input>
                <button
                    on:click=move |_| {
                        set_todos.push({
                            id: curr_id, // getting ambiguous error
                            title: input.trim().to_string(),
                            is_complete: false
                        })
                    }
                >
                    "Create todo"
                </button>
            </div>
        </div>
    }
}
