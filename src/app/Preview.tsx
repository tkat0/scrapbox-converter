import ChakraUIRenderer from "chakra-ui-markdown-renderer";
import React from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

interface PreviewProps {
  markdown: string;
}

export const Preview: React.FC<PreviewProps> = (props) => {
  const { markdown } = props;
  return (
    <ReactMarkdown
      components={ChakraUIRenderer()}
      children={markdown}
      skipHtml
      remarkPlugins={[remarkGfm]}
    />
  );
};
