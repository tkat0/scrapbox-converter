import { Box, Flex, Heading, Link, Textarea } from "@chakra-ui/react";
import { ExternalLinkIcon } from "@chakra-ui/icons";
import React, { useEffect, useState } from "react";

import { scrapboxToMarkdown } from "../main";
import { ConfigModal, defaultConfig } from "./ConfigModal";
import { defaultData } from "./data";

interface FormProps {
  value: string;
  onChange?: (event: React.ChangeEvent<HTMLTextAreaElement>) => void;
}

const Form = (props: FormProps) => {
  const { value, onChange } = props;
  return (
    <Textarea
      value={value}
      onChange={onChange}
      isReadOnly={onChange === undefined}
      h="600px"
    />
  );
};

function App() {
  const [src, setSrc] = useState(defaultData);
  const [dst, setDst] = useState(src);
  const [config, setConfig] = useState(defaultConfig);

  useEffect(() => {
    (async () => {
      const dst = await scrapboxToMarkdown(src, config);
      setDst(dst);
    })();
  }, [src, config]);

  const onChange = async (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setSrc(event.target.value);
  };

  return (
    <Box m="8">
      <Heading mb="2">Scrapbox To Markdown Converter (alpha)</Heading>
      <Box m="2">
        <Link href="https://github.com/tkat0/scrapbox-converter" isExternal>
          https://github.com/tkat0/scrapbox-converter{" "}
          <ExternalLinkIcon mx="2px" />
        </Link>
      </Box>
      <Box m="2">
        <ConfigModal config={config} setConfig={setConfig} />
      </Box>
      <Flex>
        <Box flex="1" m="2">
          <Heading size="md" mb="2">
            Scrapbox
          </Heading>
          <Form value={src} onChange={onChange} />
        </Box>
        <Box flex="1" m="2">
          <Heading size="md" mb="2">
            Markdown (read only)
          </Heading>
          <Form value={dst} />
        </Box>
      </Flex>
    </Box>
  );
}

export default App;
