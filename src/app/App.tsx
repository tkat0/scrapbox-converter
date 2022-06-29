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
  useToast,
} from "@chakra-ui/react";
import { CopyIcon, ExternalLinkIcon } from "@chakra-ui/icons";
import React, { useEffect, useState } from "react";

import { useWasm, scrapboxToMarkdown, scrapboxToAST } from "../main";
import { ConfigModal, defaultConfig } from "./ConfigModal";
import { defaultData } from "./data";
import { Header } from "./Header";
import { Preview } from "./Preview";
import { getQuery, copyQuery } from "./query";

const MAX_FORM_LENGTH = 5000;
const TOAST_ID_MAX_FORM_LENGTH = "max-form-length";

interface FormProps {
  value: string;
  onChange?: (event: React.ChangeEvent<HTMLTextAreaElement>) => void;
}

const Form = (props: FormProps) => {
  const { value, onChange } = props;
  const isReadOnly = onChange === undefined;
  const toast = useToast();

  useEffect(() => {
    if (
      !isReadOnly &&
      value.length >= MAX_FORM_LENGTH &&
      !toast.isActive(TOAST_ID_MAX_FORM_LENGTH)
    ) {
      toast({
        id: TOAST_ID_MAX_FORM_LENGTH,
        description: `Please enter within ${MAX_FORM_LENGTH} characters`,
        status: "error",
        duration: 3000,
        isClosable: true,
      });
    }
  }, [value, isReadOnly]);

  return (
    <Textarea
      value={value}
      onChange={onChange}
      isReadOnly={isReadOnly}
      maxLength={MAX_FORM_LENGTH}
      h="100%"
    />
  );
};

function App() {
  const initialized = useWasm();
  const [src, setSrc] = useState(getQuery() ?? defaultData);
  const [dst, setDst] = useState(src);
  const [ast, setAST] = useState(src);
  const [config, setConfig] = useState(defaultConfig);
  const [tabIndex, setTabIndex] = React.useState(0);

  useEffect(() => {
    if (!initialized) return;
    if (tabIndex == 2) {
      // AST
      const ast = scrapboxToAST(src);
      setAST(ast);
    } else {
      const dst = scrapboxToMarkdown(src, config);
      setDst(dst);
    }
  }, [initialized, src, config, tabIndex]);

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
      <Heading size="md">Scrapbox To Markdown Converter (alpha)</Heading>
      <Box m="2">
        <Link href="https://github.com/tkat0/scrapbox-converter" isExternal>
          https://github.com/tkat0/scrapbox-converter{" "}
          <ExternalLinkIcon mx="2px" />
        </Link>
      </Box>
      <Box m="2">
        <ConfigModal config={config} setConfig={setConfig} />
      </Box>
      <Flex flexGrow={1} flexWrap={"wrap"}>
        <Box flex="1" m="2" minW={"3xs"}>
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
        <Box flex="1" m="2" minW={"3xs"}>
          <Tabs
            display="flex"
            isFitted
            h="100%"
            flexDirection="column"
            onChange={(index) => setTabIndex(index)}
          >
            <TabList mb="1em" maxH="40px">
              <Tab>Markdown</Tab>
              <Tab>HTML</Tab>
              <Tab>AST</Tab>
            </TabList>
            <TabPanels flexGrow={1}>
              <TabPanel p="0" h="100%">
                <Form value={dst} />
              </TabPanel>
              <TabPanel p="0" h="100%">
                <Preview markdown={dst} />
              </TabPanel>
              <TabPanel p="0" h="100%">
                <Form value={ast} />
              </TabPanel>
            </TabPanels>
          </Tabs>
        </Box>
      </Flex>
    </Flex>
  );
}

export default App;
