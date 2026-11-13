//! wasm-bindgen: экспорт функций и структур в JS, импорт JS в Rust, DOM через web-sys.
use serde::Serialize;
use wasm_bindgen::prelude::*;

// ---------- 1. Импорт из JS ----------
#[wasm_bindgen]
extern "C" {
    // console.log(s) — берём функцию из глобального объекта console
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// ---------- 2. Экспорт функций ----------
/// &str на входе и String на выходе: wasm-bindgen копирует UTF-8 через линейную память.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    log(&format!("greet() вызван из JS с аргументом {name:?}"));
    format!("Привет, {name}! Длина в UTF-8: {} байт", name.len())
}

/// Result<_, JsError> превращается в исключение на стороне JS.
#[wasm_bindgen]
pub fn checked_div(a: f64, b: f64) -> Result<f64, JsError> {
    if b == 0.0 {
        return Err(JsError::new("деление на ноль"));
    }
    Ok(a / b)
}

/// Vec<f64> ↔ Float64Array: массив копируется в память wasm и обратно.
#[wasm_bindgen]
pub fn sorted(mut v: Vec<f64>) -> Vec<f64> {
    v.sort_by(f64::total_cmp);
    v
}

/// Тяжёлый цикл — кандидат на перенос в wasm: решето Эратосфена.
#[wasm_bindgen]
pub fn sum_primes(limit: u32) -> u64 {
    let n = limit as usize;
    let mut sieve = vec![true; n + 1];
    let mut sum = 0u64;
    for i in 2..=n {
        if sieve[i] {
            sum += i as u64;
            // В wasm32 usize — 32 бита: i * i переполняется при i > 65535.
            // saturating_mul даёт usize::MAX, и внутренний цикл просто не выполняется.
            let mut j = i.saturating_mul(i);
            while j <= n {
                sieve[j] = false;
                j += i;
            }
        }
    }
    sum
}

// ---------- 3. Сложные значения через serde ----------
#[derive(Serialize)]
pub struct WordCount {
    word: String,
    count: u32,
}

/// Возвращает обычный JS-массив объектов [{word, count}, ...].
#[wasm_bindgen]
pub fn top_words(text: &str, n: usize) -> Result<JsValue, JsError> {
    let mut map = std::collections::HashMap::<String, u32>::new();
    for w in text.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()) {
        *map.entry(w.to_lowercase()).or_default() += 1;
    }
    let mut v: Vec<WordCount> = map.into_iter().map(|(word, count)| WordCount { word, count }).collect();
    v.sort_by(|a, b| b.count.cmp(&a.count).then(a.word.cmp(&b.word)));
    v.truncate(n);
    Ok(serde_wasm_bindgen::to_value(&v)?)
}

// ---------- 4. Экспорт структуры как JS-класса ----------
#[wasm_bindgen]
pub struct Counter {
    value: i64,
    step: i64,
}

#[wasm_bindgen]
impl Counter {
    #[wasm_bindgen(constructor)]
    pub fn new(step: i64) -> Counter {
        Counter { value: 0, step }
    }

    pub fn increment(&mut self) -> i64 {
        self.value += self.step;
        self.value
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> i64 {
        self.value
    }
}

// ---------- 5. DOM через web-sys (только в браузере) ----------
#[wasm_bindgen]
pub fn render_list(container_id: &str, items: Vec<String>) -> Result<(), JsValue> {
    let document = web_sys::window().ok_or("нет window")?.document().ok_or("нет document")?;
    let container = document
        .get_element_by_id(container_id)
        .ok_or_else(|| JsValue::from_str("нет контейнера"))?;
    let ul = document.create_element("ul")?;
    for item in items {
        let li = document.create_element("li")?;
        li.set_text_content(Some(&item));
        ul.append_child(&li)?;
    }
    container.append_child(&ul)?;
    web_sys::console::log_1(&format!("отрисовано {} элементов", ul.child_element_count()).into());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_up_to_10() {
        assert_eq!(sum_primes(10), 2 + 3 + 5 + 7);
    }

    #[test]
    fn counter_steps() {
        let mut c = Counter::new(5);
        c.increment();
        assert_eq!(c.increment(), 10);
    }
}
