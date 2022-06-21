import { vi, describe, it, expect } from "vitest";
import { scrapboxToMarkdown } from "./main";

vi.stubGlobal("fetch", await import("fs").then((mod) => mod.readFileSync));

// it("two plus two is four", async () => {
//   await init();
//   hex_color_js("#2F14DF");
//   expect(2 + 2).toBe(4);
// });

describe(`scrapboxToMarkdown`, () => {
  it(`should convert a internal link`, async () => {
    expect(await scrapboxToMarkdown(`[internal-link]`)).toEqual(
      `[[internal-link]]`
    );
  });

  it(`should convert a code block`, async () => {
    expect(await scrapboxToMarkdown(`code:js`)).toEqual(
      `
      \`\`\`js
      \`\`\`      
      `
    );
  });
});
