import { WasmInflate, default as init } from "./pkg/wasm_inflate.js";

const file_input = document.getElementById("file-input");
const file_reader = new FileReader();
file_input.addEventListener("change", () => loadInput(file_input));

async function run() {
  // Initialize the wasm module.
  const wasm = await init("./pkg/wasm_inflate_bg.wasm");
  const wasm_inflate = WasmInflate.new();

  // Transfer archive data to wasm when the file is loaded.
  file_reader.onload = () =>
    transferContent(file_reader.result, wasm_inflate, wasm);
}

function loadInput(input) {
  const file = input.files[0];
  file_reader.readAsArrayBuffer(file);
}

// Transfer archive data to wasm when the file is loaded.
function transferContent(arrayBuffer, wasm_inflate, wasm) {
  console.log("Copying file array buffer into wasm memory ...");
  wasm_inflate.allocate(arrayBuffer.byteLength);
  const wasm_buffer = new Uint8Array(wasm.memory.buffer);
  const start = wasm_inflate.memory_pos();
  const file_buffer = new Uint8Array(arrayBuffer);
  wasm_buffer.set(file_buffer, start);
  console.log("Inflating ...");
  const t0 = performance.now();
  wasm_inflate.inflate();
  const t1 = performance.now();
  console.log("Done");
  console.log("Inflating took " + (t1 - t0) + " milliseconds.");
}

run();
