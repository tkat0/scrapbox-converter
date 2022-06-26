import {
  Box,
  Flex,
  Heading,
  IconButton,
  Link,
  Tab,
  TabList,
  TabPanel,
  TabPanels,
  Tabs,
  Textarea,
  Tooltip,
} from "@chakra-ui/react";
import { ArrowForwardIcon, CopyIcon, ExternalLinkIcon } from "@chakra-ui/icons";
import React, { useEffect, useState } from "react";

import { scrapboxToMarkdown } from "../main";
import { ConfigModal, defaultConfig } from "./ConfigModal";
import { defaultData } from "./data";
import { Header } from "./Header";
import { Preview } from "./Preview";
import { getQuery, copyQuery } from "./query";

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
      h="100%"
    />
  );
};

function App() {
  const [src, setSrc] = useState(getQuery() ?? defaultData);
  const [dst, setDst] = useState(src);
  const [config, setConfig] = useState(defaultConfig);

  useEffect(() => {
    (async () => {
      const dst = await scrapboxToMarkdown(src, config);
      setDst(dst);
    })();
  }, [src, config]);

  const onChange = async (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    const src = event.target.value;
    setSrc(src);
  };

  const onCopyClick = () => {
    copyQuery(src);
  };

  return (
    <Flex p="2" w="100vw" h="100vh" direction="column">
      <Header />
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
      <Flex alignItems="stretch" flexGrow={1}>
        <Box flex="1" m="2">
          <Tabs display="flex" isFitted h="100%" flexDirection="column">
            <TabList mb="1em" maxH="40px">
              <Tab>Scrapbox</Tab>
              <Tooltip label="Copy URL to Clipboard">
                <IconButton
                  aria-label="Copy URL to Clipboard"
                  size="sm"
                  icon={<CopyIcon />}
                  onClick={onCopyClick}
                />
              </Tooltip>
            </TabList>
            <TabPanels flexGrow={1}>
              <TabPanel p="0" h="100%">
                <Form value={src} onChange={onChange} />
              </TabPanel>
            </TabPanels>
          </Tabs>
        </Box>
        <Box>
          <ArrowForwardIcon mt="4" />
        </Box>
        <Box flex="1" m="2">
          <Tabs display="flex" isFitted h="100%" flexDirection="column">
            <TabList mb="1em" maxH="40px">
              <Tab>Markdown</Tab>
              <Tab>HTML</Tab>
            </TabList>
            <TabPanels flexGrow={1}>
              <TabPanel p="0" h="100%">
                <Form value={dst} />
              </TabPanel>
              <TabPanel p="0" h="100%">
                <Preview markdown={dst} />
              </TabPanel>
            </TabPanels>
          </Tabs>
        </Box>
      </Flex>
    </Flex>
  );
}

export default App;
