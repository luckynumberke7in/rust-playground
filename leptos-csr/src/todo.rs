use leptos::*;

#[component]
pub fn Todo(cx: Scope) -> impl IntoView {
    let static_todos = vec![
        "make normal list with .map --done",
        "make signal for todos (vec containing a string, bool, and id --done",
        "make todo input and add todo button --done",
        "make toggle todo, delete, & maybe edit",
        "make for component to iterate over them -- partially done",
    ];
    #[derive(Clone)]
    struct TodoData {
        id: usize,
        title: String,
        is_complete: bool,
    }

    let (todos, set_todos) = create_signal(cx, Vec::new());

    let mut new_id = 0;
    let (input, set_input) = create_signal(cx, "".to_string());

    let add_todo = move |_| {
        set_todos.update(move |todos| {
            todos.push(TodoData {
                id: new_id,
                title: input().trim().to_string(),
                is_complete: false,
            })
        });
        set_input("".to_string());
        new_id += 1;
    };
    let remove_todo = move |todo: &TodoData| {
        let remove_id = todo.id;
        set_todos.update(move |todos| todos.retain(|todo| todo.id != remove_id));
    };

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
                    on:click=add_todo
                >
                    "Add todo"
                </button>
            </div>
            <ul>
                <For // map out each todo, zooming scope to allow manipulation of each todo individually
                    each=todos
                    key=|todo| todo.id
                    view=move |cx, todo| {
                        view! { cx,
                            <li>
                                {&todo.title}
                                <button
                                    on:click=move |_| remove_todo(&todo)
                                >
                                    "Delete"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
