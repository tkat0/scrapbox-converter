import { vi, describe, expect, beforeAll, test } from "vitest";
import { defaultConfig } from "./app/ConfigModal";
import { init, scrapboxToMarkdown } from "./main";
import fs from "fs";

vi.stubGlobal("fetch", await import("fs").then((mod) => mod.readFileSync));

describe(`scrapboxToMarkdown`, () => {
  beforeAll(async () => {
    await init();
  });

  test(`Scrapboxの使い方`, () => {
    const input = fs.readFileSync(
      "./src/__test__/help-jp/Scrapboxの使い方_input.txt",
      "utf8"
    );
    const expected = fs.readFileSync(
      "./src/__test__/help-jp/Scrapboxの使い方_expected.txt",
      "utf8"
    );
    expect(scrapboxToMarkdown(input, defaultConfig)).toEqual(expected);
  });
});
