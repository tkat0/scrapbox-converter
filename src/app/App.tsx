import { Box, Flex, Heading, Link, Textarea } from "@chakra-ui/react";
import { ExternalLinkIcon } from "@chakra-ui/icons";
import React, { useEffect, useState } from "react";

import { scrapboxToMarkdown } from "../main";

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
      h="300px"
    />
  );
};

function App() {
  const [src, setSrc] = useState("[Scrapbox https://scrapbox.io]");
  const [dst, setDst] = useState(src);

  useEffect(() => {
    (async () => {
      const dst = await scrapboxToMarkdown(src);
      setDst(dst);
    })();
  }, [src]);

  const onChange = async (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setSrc(event.target.value);
  };

  return (
    <Box m="8">
      <Heading mb="2">Scrapbox To Markdown Converter (alpha)</Heading>
      <Link href="https://github.com/tkat0/scrapbox-converter" isExternal>
        https://github.com/tkat0/scrapbox-converter{" "}
        <ExternalLinkIcon mx="2px" />
      </Link>
      <Flex mt="8">
        <Box flex="1" mr="1" ml="1">
          <Heading size="md">Scrapbox</Heading>
          <Form value={src} onChange={onChange} />
        </Box>
        <Box flex="1" mr="1" ml="1">
          <Heading size="md">Markdown (read only)</Heading>
          <Form value={dst} />
        </Box>
      </Flex>
    </Box>
  );
}

export default App;
