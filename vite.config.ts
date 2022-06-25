import path from "path";
import { defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";

module.exports = defineConfig({
  base: "/scrapbox-converter/",
  build: {
    // TODO(tkat0): currently, library-mode is not supported.
    // https://github.com/vitejs/vite/issues/3001
    // https://vitejs.dev/guide/build.html#library-mode
    // lib: {
    //   entry: path.resolve(__dirname, "src/main.ts"),
    //   name: "scrapbox-converter",
    //   fileName: (format) => `scrapbox-converter.${format}.js`,
    // },
    // rollupOptions: {
    //   external: ["react"],
    //   output: {
    //     globals: {
    //       vue: "React",
    //     },
    //   },
    // },
  },
  test: {
    environment: "jsdom",
  },
  plugins: [react(), tsconfigPaths()],
});
