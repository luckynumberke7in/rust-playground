use leptos::*;

#[component]
pub fn Todo(cx: Scope) -> impl IntoView {
    let static_todos = vec![
        "make normal list with .map --done",
        "make signal for todos (vec containing a string, bool, and id --done",
        "make todo input and add todo button --done",
        "make toggle todo, delete, & maybe edit",
        "make for component to iterate over them",
    ];
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
        new_id += 1;
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
                <For // fix me! not accessing todos, set_todos or the .id correctly!
                    each=todos
                    key=|todo| todo.id
                    view=move |cx, (id, (todos, set_todos))| {
                        view! { cx,
                            <li>
                                {title}
                                <button
                                    on:click= |_| {
                                        set_todos.update(|todos| {
                                            todos.retain((|todo_id, _)| todo_id != &id)
                                        });
                                    }
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
