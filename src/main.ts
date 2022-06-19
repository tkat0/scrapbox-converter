import init, { scrapbox_to_markdown } from "@@/scrapbox_parser_core";

export const scrapboxToMarkdown = async (input: string): Promise<string> => {
  await init();
  return scrapbox_to_markdown(input);
};
