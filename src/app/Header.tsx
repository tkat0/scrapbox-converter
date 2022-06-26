import React from "react";
import { Helmet } from "react-helmet-async";

const GA_MEASUREMENT_ID = "G-N0JP3MESQQ";
const DEBUG_MODE = import.meta.env.DEV;

export const Header: React.FC = () => {
  // https://developers.google.com/analytics/devguides/collection/gtagjs/
  return (
    <Helmet>
      <script
        async
        src={`https://www.googletagmanager.com/gtag/js?id=${GA_MEASUREMENT_ID}`}
      ></script>
      <script>
        {`
          window.dataLayer = window.dataLayer || [];
          function gtag(){window.dataLayer.push(arguments);}
          gtag('js', new Date());
          
          gtag('config', '${GA_MEASUREMENT_ID}', {'debug_mode': ${DEBUG_MODE}});
        `}
      </script>
    </Helmet>
  );
};
