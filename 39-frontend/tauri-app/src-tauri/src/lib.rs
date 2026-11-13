//! Бэкенд Tauri: команды, состояние, события.
use std::sync::Mutex;
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Builder, Emitter, Runtime, State};

/// Состояние приложения: живёт в Rust, общее для всех окон.
#[derive(Default)]
pub struct Notes(Mutex<Vec<String>>);

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Привет, {name}! Ответ пришёл из Rust-процесса")
}

/// Result<_, String>: Err превращается в rejected Promise во фронтенде.
#[tauri::command]
fn add_note(text: String, notes: State<'_, Notes>) -> Result<Vec<String>, String> {
    let text = text.trim();
    if text.is_empty() {
        return Err("пустую заметку сохранить нельзя".into());
    }
    let mut list = notes.0.lock().map_err(|e| e.to_string())?;
    list.push(text.to_owned());
    Ok(list.clone())
}

#[derive(Serialize)]
struct SysInfo {
    os: &'static str,
    arch: &'static str,
    tauri: &'static str,
    pid: u32,
}

#[tauri::command]
fn system_info() -> SysInfo {
    SysInfo {
        os: std::env::consts::OS,
        arch: std::env::consts::ARCH,
        tauri: tauri::VERSION,
        pid: std::process::id(),
    }
}

/// async-команда выполняется в tokio-рантайме Tauri и шлёт события в окно.
#[tauri::command]
async fn countdown<R: Runtime>(app: AppHandle<R>, from: u32) -> Result<(), String> {
    for n in (0..=from).rev() {
        app.emit("tick", n).map_err(|e| e.to_string())?;
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    Ok(())
}

/// Сборка вынесена отдельно, чтобы тесты могли подставить mock-рантайм.
pub fn setup<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder
        .manage(Notes::default())
        .invoke_handler(tauri::generate_handler![greet, add_note, system_info, countdown])
}

pub fn run() {
    setup(tauri::Builder::default())
        .run(tauri::generate_context!())
        .expect("ошибка запуска приложения");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tauri::ipc::{CallbackFn, InvokeBody};
    use tauri::test::{INVOKE_KEY, get_ipc_response, mock_builder, mock_context, noop_assets};
    use tauri::webview::InvokeRequest;

    fn request(cmd: &str, body: serde_json::Value) -> InvokeRequest {
        InvokeRequest {
            cmd: cmd.into(),
            callback: CallbackFn(0),
            error: CallbackFn(1),
            url: "tauri://localhost".parse().unwrap(),
            body: InvokeBody::Json(body),
            headers: Default::default(),
            invoke_key: INVOKE_KEY.to_string(),
        }
    }

    #[test]
    fn commands_through_ipc() {
        let app = setup(mock_builder()).build(mock_context(noop_assets())).unwrap();
        let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
            .build()
            .unwrap();

        let r = get_ipc_response(&webview, request("greet", serde_json::json!({ "name": "OTUS" })));
        assert_eq!(r.unwrap().deserialize::<String>().unwrap(), "Привет, OTUS! Ответ пришёл из Rust-процесса");

        let r = get_ipc_response(&webview, request("add_note", serde_json::json!({ "text": "  " })));
        assert_eq!(r.unwrap_err(), serde_json::json!("пустую заметку сохранить нельзя"));

        let r = get_ipc_response(&webview, request("add_note", serde_json::json!({ "text": "wasm" })));
        assert_eq!(r.unwrap().deserialize::<Vec<String>>().unwrap(), vec!["wasm"]);
    }
}
