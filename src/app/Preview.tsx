import { Link } from "@chakra-ui/react";
import ChakraUIRenderer from "chakra-ui-markdown-renderer";
import React from "react";
import ReactMarkdown, { Components } from "react-markdown";
import remarkGfm from "remark-gfm";

const newTheme: Components = {
  a: (props) => {
    const { href, children } = props;
    return (
      <Link href={href} isExternal color="teal.500">
        {children}
      </Link>
    );
  },
};

interface PreviewProps {
  markdown: string;
}

export const Preview: React.FC<PreviewProps> = (props) => {
  const { markdown } = props;
  return (
    <ReactMarkdown
      components={ChakraUIRenderer(newTheme)}
      children={markdown}
      skipHtml
      remarkPlugins={[remarkGfm]}
    />
  );
};
