import { useEffect, useState } from "react";

import initCore, {
  scrapboxToMarkdown as scrapboxToMarkdownCore,
  scrapboxToAST as scrapboxToASTCore,
  markdownToScrapbox as markdownToScrapboxCore,
  markdownToAST as markdownToASTCore,
  Config,
} from "@@/scrapbox_converter_core";

export const init = async () => {
  await initCore();
};

export const scrapboxToMarkdown = (input: string, config: Config): string => {
  try {
    return scrapboxToMarkdownCore(input, config);
  } catch (error) {
    console.error(error);
    return "";
  }
};

export const scrapboxToAST = (input: string, config: Config): string => {
  try {
    return scrapboxToASTCore(input, config);
  } catch (error) {
    console.error(error);
    return "";
  }
};

export const markdownToScrapbox = (input: string, config: Config): string => {
  try {
    return markdownToScrapboxCore(input, config);
  } catch (error) {
    console.error(error);
    return "";
  }
};

export const markdownToAST = (input: string, config: Config): string => {
  try {
    return markdownToASTCore(input, config);
  } catch (error) {
    console.error(error);
    return "";
  }
};

export const useWasm = (): boolean => {
  const [initialized, setInitialized] = useState(false);

  useEffect(() => {
    init()
      .then(() => {
        setInitialized(true);
      })
      .catch((reason) => {
        console.error(reason);
      });
  }, []);

  return initialized;
};
