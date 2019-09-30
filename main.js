import init, { run_app } from './pkg/rust_text_adventure.js';
async function main() {
   await init('/pkg/rust_text_adventure_bg.wasm');
   run_app();
}
main()
