import init, { run_app } from './pkg/rust_text_adventure.js';
async function main() {
   await init('/pkg/yew_wasm_pack_minimal_bg.wasm');
   run_app();
}
main()
