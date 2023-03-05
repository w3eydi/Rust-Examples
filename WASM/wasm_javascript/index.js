import init from "./pkg/wasm_javascript.js";

const runWasm = async () => {
    // Instantiate our wasm module
    const wasmJavascript = await init("./pkg/wasm_javascript_bg.wasm");
  
    // Call the Add function export from wasm, save the result
    const addResult = wasmJavascript.add(24, 24);
  
    // Set the result onto the body
    document.body.textContent = `Hello wasm from pure javascript! addResult: ${addResult}`;
};
runWasm();
  