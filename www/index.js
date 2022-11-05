import init, { great } from "wa-snake";

async function start() {
  const wasm = await init();
  great("Wojtek");
  console.log("Hello");
}
start()
