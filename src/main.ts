import { useEffect, useState } from "react";

import initCore, {
  scrapboxToMarkdown as scrapboxToMarkdownCore,
} from "@@/scrapbox_converter_core";
import { Config } from "./app/ConfigModal";

export const init = async () => {
  await initCore();
};

export const scrapboxToMarkdown = (input: string, config: Config): string => {
  return scrapboxToMarkdownCore(input, config);
};

export const useScrapboxToMarkdown = (): boolean => {
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
