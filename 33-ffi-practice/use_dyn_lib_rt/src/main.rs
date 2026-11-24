use std::path::PathBuf;

type MyAddFn = unsafe extern "C" fn(left: u64, right: u64) -> u64;

fn main() {
    let target = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target"));
    let path = target
        .join("debug")
        .join(libloading::library_filename("my_lib"));

    // SAFETY: библиотека собрана из локального `my_lib`; загруженный символ
    // используется только пока жив объект `lib`.
    let lib = match unsafe { libloading::Library::new(&path) } {
        Ok(lib) => lib,
        Err(error) => {
            eprintln!("Не удалось загрузить {}: {error}", path.display());
            return;
        }
    };

    // SAFETY: `my_lib` экспортирует `add` именно с сигнатурой MyAddFn.
    let add_fn = match unsafe { lib.get::<MyAddFn>(b"add") } {
        Ok(add_fn) => add_fn,
        Err(error) => {
            eprintln!("Не удалось загрузить функцию add: {error}");
            return;
        }
    };

    // SAFETY: указатель получен из совместимой функции, аргументы — обычные u64.
    let sum = unsafe { add_fn(1, 2) };
    println!("Hello, dyn rt sum: {sum}!");
}
