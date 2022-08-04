import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';
import { resolve } from 'path';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [svelte()],

    resolve: {
        alias: [
            {
                find: '$lib',
                replacement: resolve('./src/lib'),
            },
        ],
    },
});
