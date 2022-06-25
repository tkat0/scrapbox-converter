import { vi, describe, it, expect } from "vitest";
import { defaultConfig } from "./app/ConfigModal";
import { scrapboxToMarkdown } from "./main";

vi.stubGlobal("fetch", await import("fs").then((mod) => mod.readFileSync));

describe(`scrapboxToMarkdown`, () => {
  it(`should convert a internal link`, async () => {
    expect(await scrapboxToMarkdown(`[internal-link]`, defaultConfig)).toEqual(
      `[[internal-link]]\n`
    );
  });
});
