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

import {
  useWasm,
  scrapboxToMarkdown,
  scrapboxToAST,
  markdownToScrapbox,
  markdownToAST,
} from "../main";
import { ConfigModal, defaultConfig } from "./ConfigModal";
import { defaultScrapboxData, defaultMarkdownData } from "./data";
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
  const query = getQuery();
  const [scrapboxSrc, setScrapboxSrc] = useState(
    query.form ?? defaultScrapboxData
  );
  const [markdownSrc, setMarkdownSrc] = useState(
    query.form ?? defaultMarkdownData
  );
  const [dst, setDst] = useState(scrapboxSrc);
  const [config, setConfig] = useState(defaultConfig);
  const [srcTabIndex, setSrcTabIndex] = React.useState(query.tabIndex ?? 0);
  const [dstTabIndex, setDstTabIndex] = React.useState(query.tabIndex ?? 0);

  useEffect(() => {
    if (!initialized) return;

    const mapping: ((() => void) | null)[][] = [
      [
        // scrapbox to
        () => {
          const dst = scrapboxToMarkdown(scrapboxSrc, config);
          setDst(dst);
        },
        null,
        () => {
          const dst = scrapboxToMarkdown(scrapboxSrc, config);
          setDst(dst);
        },
        () => {
          const dst = scrapboxToAST(scrapboxSrc);
          setDst(dst);
        },
      ],
      [
        // markdown to
        null,
        () => {
          const dst = markdownToScrapbox(markdownSrc);
          setDst(dst);
        },
        () => {
          // show markdown directly
          setDst(markdownSrc);
        },
        () => {
          const dst = markdownToAST(markdownSrc);
          setDst(dst);
        },
      ],
    ];

    const f = mapping[srcTabIndex][dstTabIndex];
    f && f();
  }, [initialized, scrapboxSrc, markdownSrc, config, srcTabIndex, dstTabIndex]);

  const onScraobpxFormChange = async (
    event: React.ChangeEvent<HTMLTextAreaElement>
  ) => {
    const src = event.target.value;
    setScrapboxSrc(src);
  };

  const onMarkdownFormChange = async (
    event: React.ChangeEvent<HTMLTextAreaElement>
  ) => {
    const src = event.target.value;
    setMarkdownSrc(src);
  };

  const onSrcTabChange = (index: number) => {
    setSrcTabIndex(index);
    if ([2, 3].includes(dstTabIndex)) return;
    switch (index) {
      case 0:
        setDstTabIndex(0);
        break;
      case 1:
        setDstTabIndex(1);
        break;
      default:
        break;
    }
  };
  const onDstTabChange = (index: number) => {
    setDstTabIndex(index);
    switch (index) {
      case 0:
        setSrcTabIndex(0);
        break;
      case 1:
        setSrcTabIndex(1);
        break;
      default:
        break;
    }
  };

  const onCopyClick = () => {
    copyQuery(scrapboxSrc, srcTabIndex);
  };

  return (
    <Flex p="2" w="100vw" h="100vh" direction="column">
      <Header />
      <Heading size="md">
        <Link href={`${location.origin}${location.pathname}`}>
          Scrapbox To Markdown Converter (alpha)
        </Link>
      </Heading>
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
          <Tabs
            display="flex"
            isFitted
            h="100%"
            flexDirection="column"
            index={srcTabIndex}
            onChange={onSrcTabChange}
          >
            <TabList mb="1em" maxH="40px">
              <Tab>Scrapbox</Tab>
              <Tab>Markdown</Tab>
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
                <Form value={scrapboxSrc} onChange={onScraobpxFormChange} />
              </TabPanel>
              <TabPanel p="0" h="100%">
                <Form value={markdownSrc} onChange={onMarkdownFormChange} />
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
            index={dstTabIndex}
            onChange={onDstTabChange}
          >
            <TabList mb="1em" maxH="40px">
              <Tab>Markdown</Tab>
              <Tab>Scrapbox</Tab>
              <Tab>HTML</Tab>
              <Tab>AST</Tab>
            </TabList>
            <TabPanels flexGrow={1}>
              <TabPanel p="0" h="100%">
                <Form value={dst} />
              </TabPanel>
              <TabPanel p="0" h="100%">
                <Form value={dst} />
              </TabPanel>
              <TabPanel p="0" h="100%">
                <Preview markdown={dst} />
              </TabPanel>
              <TabPanel p="0" h="100%">
                <Form value={dst} />
              </TabPanel>
            </TabPanels>
          </Tabs>
        </Box>
      </Flex>
    </Flex>
  );
}

export default App;
