import { vi, describe, it, expect } from "vitest";
import { scrapboxToMarkdown } from "./main";

vi.stubGlobal("fetch", await import("fs").then((mod) => mod.readFileSync));

describe(`scrapboxToMarkdown`, () => {
  it(`should convert a internal link`, async () => {
    expect(await scrapboxToMarkdown(`[internal-link]`)).toEqual(
      `[[internal-link]]`
    );
  });
});
