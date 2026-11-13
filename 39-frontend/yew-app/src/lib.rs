//! Yew-приложение: компоненты, свойства, колбэки, хуки.
use std::rc::Rc;

use gloo_timers::callback::Interval;
use web_sys::HtmlInputElement;
use yew::prelude::*;

// ---------- 1. Состояние: use_state ----------
#[component]
pub fn Counter() -> Html {
    let count = use_state(|| 0);
    // Callback — клонируемая обёртка над Rc<dyn Fn>
    let inc = {
        let count = count.clone();
        Callback::from(move |_: MouseEvent| count.set(*count + 1))
    };
    html! {
        <section class="card">
            <h2>{ "Счётчик" }</h2>
            <button onclick={inc}>{ "+1" }</button>
            <span class="value">{ *count }</span>
        </section>
    }
}

// ---------- 2. Редьюсер + дочерний компонент со свойствами ----------
#[derive(Clone, PartialEq)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub done: bool,
}

#[derive(Default, PartialEq)]
pub struct Todos {
    next_id: u32,
    pub items: Vec<Todo>,
}

pub enum TodoAction {
    Add(String),
    Toggle(u32),
}

impl Reducible for Todos {
    type Action = TodoAction;
    fn reduce(self: Rc<Self>, action: TodoAction) -> Rc<Self> {
        let mut items = self.items.clone();
        let mut next_id = self.next_id;
        match action {
            TodoAction::Add(text) => {
                next_id += 1;
                items.push(Todo { id: next_id, text, done: false });
            }
            TodoAction::Toggle(id) => {
                if let Some(t) = items.iter_mut().find(|t| t.id == id) {
                    t.done = !t.done;
                }
            }
        }
        Rc::new(Todos { next_id, items })
    }
}

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
    pub todo: Todo,
    pub on_toggle: Callback<u32>,
}

/// Перерисовывается, только если изменились свойства (PartialEq).
#[component]
pub fn TodoItem(props: &TodoItemProps) -> Html {
    let id = props.todo.id;
    let onclick = props.on_toggle.reform(move |_: MouseEvent| id);
    html! {
        <li class={classes!(props.todo.done.then_some("done"))} {onclick}>
            { &props.todo.text }
        </li>
    }
}

#[component]
pub fn TodoList() -> Html {
    let todos = use_reducer(|| Todos {
        next_id: 1,
        items: vec![Todo { id: 1, text: "Собрать wasm".into(), done: true }],
    });
    let input = use_node_ref();

    let on_add = {
        let (todos, input) = (todos.clone(), input.clone());
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let el = input.cast::<HtmlInputElement>().expect("input");
            let text = el.value().trim().to_owned();
            if !text.is_empty() {
                todos.dispatch(TodoAction::Add(text));
                el.set_value("");
            }
        })
    };
    let on_toggle = {
        let todos = todos.clone();
        Callback::from(move |id| todos.dispatch(TodoAction::Toggle(id)))
    };
    let left = todos.items.iter().filter(|t| !t.done).count();

    html! {
        <section class="card">
            <h2>{ format!("Задачи (осталось {left})") }</h2>
            <form onsubmit={on_add}>
                <input ref={input} placeholder="Новая задача" />
                <button type="submit">{ "Добавить" }</button>
            </form>
            <ul>
                { for todos.items.iter().map(|t| html! {
                    <TodoItem key={t.id} todo={t.clone()} on_toggle={on_toggle.clone()} />
                }) }
            </ul>
        </section>
    }
}

// ---------- 3. Эффект с очисткой: use_effect_with ----------
#[component]
pub fn Clock() -> Html {
    let seconds = use_state(|| 0u32);
    {
        let seconds = seconds.clone();
        // deps = () → эффект один раз при монтировании, деструктор при размонтировании
        use_effect_with((), move |_| {
            let interval = Interval::new(1000, move || seconds.set(*seconds + 1));
            move || drop(interval)
        });
    }
    html! { <p class="clock">{ format!("На странице: {} с", *seconds) }</p> }
}

#[component]
pub fn App() -> Html {
    html! {
        <main>
            <h1>{ "Yew: фронтенд на Rust" }</h1>
            <Clock />
            <Counter />
            <TodoList />
        </main>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// SSR в обычном тесте: рендерим компонент в строку без браузера.
    #[tokio::test]
    async fn renders_initial_html() {
        let html = yew::LocalServerRenderer::<App>::new().render().await;
        assert!(html.contains("Yew: фронтенд на Rust"));
        assert!(html.contains("Задачи (осталось 0)"));
        assert!(html.contains("Собрать wasm"));
    }

    #[test]
    fn reducer_adds_and_toggles() {
        let s = Rc::new(Todos::default());
        let s = s.reduce(TodoAction::Add("a".into()));
        let id = s.items[0].id;
        let s = s.reduce(TodoAction::Toggle(id));
        assert!(s.items[0].done);
    }
}
