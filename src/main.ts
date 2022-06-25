import init, {
  scrapboxToMarkdown as scrapboxToMarkdownCore,
} from "@@/scrapbox_converter_core";
import { Config } from "./app/ConfigModal";

export const scrapboxToMarkdown = async (
  input: string,
  config: Config
): Promise<string> => {
  await init();
  return scrapboxToMarkdownCore(input, config);
};
