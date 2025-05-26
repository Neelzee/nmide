import { DebugCore } from "@nmide/js-utils";
import { test, expect } from "vitest";
import { from_js_core_to_gleam_core } from "../build/dev/javascript/nmide_gleam_utils/gleam/decoder/core.mjs";
import { Core as GleamCore } from "../build/dev/javascript/nmide_gleam_utils/gleam/utils/core.mjs";

test("DebugCore Translation", () => {
  const debugcore = DebugCore();
  const gleam_core = from_js_core_to_gleam_core(debugcore);
  console.log(gleam_core);
  expect(gleam_core).toBeInstanceOf(GleamCore);
});