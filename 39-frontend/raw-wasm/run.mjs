// node run.mjs — загрузка модуля без всякого клея
import fs from "node:fs";
const bytes = fs.readFileSync(import.meta.dirname + "/add.wasm");
const mod = new WebAssembly.Module(bytes);
console.log("размер:", bytes.length, "байт");
console.log("exports:", WebAssembly.Module.exports(mod).map(e => `${e.name}:${e.kind}`).join(", "));
console.log("imports:", WebAssembly.Module.imports(mod).length);

const { exports } = new WebAssembly.Instance(mod, {});
console.log("add(2, 3) =", exports.add(2, 3));
console.log("add(2**31 - 1, 1) =", exports.add(2 ** 31 - 1, 1)); // i32 переполняется
console.log("hello_ptr() =", exports.hello_ptr());              // просто число!

// Чтобы получить строку, читаем линейную память руками
const view = new Uint8Array(exports.memory.buffer, exports.hello_ptr(), exports.hello_len());
console.log("строка из памяти:", new TextDecoder().decode(view));
