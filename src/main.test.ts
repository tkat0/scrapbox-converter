import { vi, describe, it, expect, beforeAll } from "vitest";
import { defaultConfig } from "./app/ConfigModal";
import { init, scrapboxToMarkdown } from "./main";

vi.stubGlobal("fetch", await import("fs").then((mod) => mod.readFileSync));

describe(`scrapboxToMarkdown`, () => {
  beforeAll(async () => {
    await init();
  });

  it(`should convert a internal link`, () => {
    expect(scrapboxToMarkdown(`[internal-link]`, defaultConfig)).toEqual(
      `[[internal-link]]\n`
    );
  });
});
