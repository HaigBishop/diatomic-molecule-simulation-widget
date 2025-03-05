import init, { greet, add_numbers } from "../public/wasm/wasm_crate.js";

async function runWasm() {
    await init();

    console.log(greet("World"));

    const result = add_numbers(24, 42);

    document.body.textContent = `add result ${result}`;
}

runWasm();
