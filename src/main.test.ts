import { vi, it, expect } from "vitest";
import init, { hex_color_js } from "../pkg/web/scrapbox-parser-core-web";

vi.stubGlobal("fetch", await import("fs").then((mod) => mod.readFileSync));

it("two plus two is four", async () => {
  await init();
  hex_color_js("#2F14DF");
  expect(2 + 2).toBe(4);
});
