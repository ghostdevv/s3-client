import { svelte } from '@sveltejs/vite-plugin-svelte';
import Terminal from 'vite-plugin-terminal';
import { defineConfig } from 'vite';
import { resolve } from 'path';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [svelte(), Terminal({ console: 'terminal' })],

    resolve: {
        alias: [
            {
                find: '$lib',
                replacement: resolve('./src/lib'),
            },
        ],
    },
});
