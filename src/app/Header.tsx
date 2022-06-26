import React from "react";
import { Helmet } from "react-helmet-async";

export const Header: React.FC = () => {
  return (
    <Helmet>
      <title>Scrapbox To Markdown Converter (alpha)</title>
    </Helmet>
  );
};
