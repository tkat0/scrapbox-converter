const path = require('path');
import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';

// https://vitejs.dev/guide/build.html#library-mode
module.exports = defineConfig({
    build: {
        lib: {
            entry: path.resolve(__dirname, 'src/main.ts'),
            name: 'scrapbox-parser',
            fileName: (format) => `scrapbox-parser.${format}.js`,
        },
        rollupOptions: {
            // make sure to externalize deps that shouldn't be bundled
            // into your library
            external: ['react'],
            output: {
                // Provide global variables to use in the UMD build
                // for externalized deps
                globals: {
                    vue: 'React',
                },
            },
        },
    },
    test: {
        environment: 'jsdom',
    },
    plugins: [react()],
});
