import ready, { setup, run } from "./pkg/mod.js";

await ready();
setup();
const out = run();
console.log('out', out);
