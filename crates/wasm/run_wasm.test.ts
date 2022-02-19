import { assert } from "https://deno.land/std@0.126.0/testing/asserts.ts";
import ready, { setup, parse } from "./pkg/mod.js";

await ready();
setup();

Deno.test("parse", () => {
  const parsed = parse("#ff3c4c");
  assert(parsed.r === 255);
  assert(parsed.g === 60);
  assert(parsed.b === 76);
});
