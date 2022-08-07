import { vi, describe, expect, beforeAll, test } from "vitest";
import { defaultConfig } from "./app/ConfigModal";
import { init, scrapboxToMarkdown } from "./main";
import fs from "fs";

vi.stubGlobal("fetch", await import("fs").then((mod) => mod.readFileSync));

describe(`scrapboxToMarkdown`, () => {
  beforeAll(async () => {
    await init();
  });

  const genTest = (path: string) => {
    test(path, () => {
      const input = fs.readFileSync(`./src/__test__${path}_input.txt`, "utf8");
      const expected = fs.readFileSync(
        `./src/__test__${path}_expected.txt`,
        "utf8"
      );
      expect(scrapboxToMarkdown(input, defaultConfig)).toEqual(expected);
    });
  };

  // https://scrapbox.io/help-jp/
  genTest(`/help-jp/Scrapboxの使い方`);
  genTest(`/help-jp/コードブロック`);
});
