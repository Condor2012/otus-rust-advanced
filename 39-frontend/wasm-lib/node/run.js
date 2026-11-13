// node node/run.js — после: wasm-bindgen --target nodejs --out-dir pkg-node ...
const wasm = require("../pkg-node/wasm_lib.js");

console.log("1)", wasm.greet("OTUS"));

try {
  wasm.checked_div(1, 0);
} catch (e) {
  console.log("2) исключение из Rust:", e.message);
}

console.log("3) sorted:", wasm.sorted(new Float64Array([3.5, -1, 2, 10])));
console.log("4) top_words:", wasm.top_words("Rust и wasm, wasm и Rust, и снова wasm", 3));

const c = new wasm.Counter(5n); // i64 ↔ BigInt
c.increment();
c.increment();
console.log("5) Counter.value =", c.value);
c.free(); // память структуры живёт в wasm — освобождаем явно

// 6) Тот же алгоритм на JS
function sumPrimesJs(n) {
  const sieve = new Uint8Array(n + 1).fill(1);
  let sum = 0;
  for (let i = 2; i <= n; i++) {
    if (sieve[i]) {
      sum += i;
      for (let j = i * i; j <= n; j += i) sieve[j] = 0;
    }
  }
  return sum;
}
const N = 20_000_000;
for (const [name, f] of [["wasm", () => wasm.sum_primes(N)], ["js  ", () => sumPrimesJs(N)]]) {
  f(); // прогрев
  const t = performance.now();
  const r = f();
  console.log(`6) ${name} sum_primes(${N}) = ${r}  ${(performance.now() - t).toFixed(1)} ms`);
}
