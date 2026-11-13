//! Фронтенд Tauri-приложения на Yew. Вызывает Rust-бэкенд через window.__TAURI__.
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

// ---------- мост к Tauri через wasm-bindgen ----------
#[wasm_bindgen]
extern "C" {
    /// window.__TAURI__.core.invoke(cmd, args) → Promise; catch превращает reject в Err
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    /// window.__TAURI__.event.listen(event, handler)
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> JsValue;
}

/// Типизированная обёртка: сериализуем аргументы и десериализуем ответ.
async fn call<A: Serialize, R: for<'de> Deserialize<'de>>(cmd: &str, args: &A) -> Result<R, String> {
    let args = serde_wasm_bindgen::to_value(args).map_err(|e| e.to_string())?;
    match invoke(cmd, args).await {
        Ok(v) => serde_wasm_bindgen::from_value(v).map_err(|e| e.to_string()),
        Err(e) => Err(e.as_string().unwrap_or_else(|| format!("{e:?}"))),
    }
}

#[derive(Serialize)]
struct GreetArgs<'a> {
    name: &'a str,
}
#[derive(Serialize)]
struct NoteArgs {
    text: String,
}
#[derive(Serialize)]
struct CountdownArgs {
    from: u32,
}
#[derive(Serialize)]
struct NoArgs {}

#[derive(Deserialize, Clone, PartialEq, Default)]
struct SysInfo {
    os: String,
    arch: String,
    tauri: String,
    pid: u32,
}

#[derive(Deserialize)]
struct TickEvent {
    payload: u32,
}

#[component]
fn App() -> Html {
    let greeting = use_state(String::new);
    let info = use_state(SysInfo::default);
    let notes = use_state(Vec::<String>::new);
    let error = use_state(|| None::<String>);
    let tick = use_state(|| None::<u32>);
    let input = use_node_ref();

    // При монтировании: два вызова команд и подписка на событие
    {
        let (greeting, info, tick) = (greeting.clone(), info.clone(), tick.clone());
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(s) = call::<_, String>("greet", &GreetArgs { name: "OTUS" }).await {
                    greeting.set(s);
                }
                if let Ok(i) = call::<_, SysInfo>("system_info", &NoArgs {}).await {
                    info.set(i);
                }
                let handler = Closure::<dyn FnMut(JsValue)>::new(move |e: JsValue| {
                    if let Ok(t) = serde_wasm_bindgen::from_value::<TickEvent>(e) {
                        tick.set(Some(t.payload));
                    }
                });
                listen("tick", &handler).await;
                handler.forget(); // обработчик живёт всё время работы окна
            });
        });
    }

    let on_add = {
        let (notes, error, input) = (notes.clone(), error.clone(), input.clone());
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let el = input.cast::<HtmlInputElement>().unwrap();
            let text = el.value();
            let (notes, error) = (notes.clone(), error.clone());
            spawn_local(async move {
                match call::<_, Vec<String>>("add_note", &NoteArgs { text }).await {
                    Ok(list) => {
                        notes.set(list);
                        error.set(None);
                    }
                    Err(msg) => error.set(Some(msg)),
                }
            });
            el.set_value("");
        })
    };

    let on_countdown = Callback::from(|_: MouseEvent| {
        spawn_local(async {
            let _ = call::<_, ()>("countdown", &CountdownArgs { from: 5 }).await;
        });
    });

    html! {
        <main>
            <h1>{ "Tauri 2 + Yew" }</h1>
            <section class="card">
                <h2>{ "invoke(\"greet\")" }</h2>
                <p>{ (*greeting).clone() }</p>
            </section>
            <section class="card">
                <h2>{ "invoke(\"system_info\")" }</h2>
                <p>{ format!("ОС: {} · {} · Tauri {} · PID {}", info.os, info.arch, info.tauri, info.pid) }</p>
            </section>
            <section class="card">
                <h2>{ "Состояние: State<Notes>" }</h2>
                <form onsubmit={on_add}>
                    <input ref={input} placeholder="Заметка" />
                    <button type="submit">{ "Сохранить в Rust" }</button>
                </form>
                if let Some(msg) = &*error { <p class="error">{ format!("Ошибка из Rust: {msg}") }</p> }
                <ul>{ for notes.iter().map(|n| html! { <li>{ n }</li> }) }</ul>
            </section>
            <section class="card">
                <h2>{ "События: emit(\"tick\")" }</h2>
                <button onclick={on_countdown}>{ "Обратный отсчёт" }</button>
                <span class="tick">{ tick.map(|t| t.to_string()).unwrap_or_default() }</span>
            </section>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
