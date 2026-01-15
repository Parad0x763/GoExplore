import { defineConfig } from 'vite'
import preact from '@preact/preset-vite'

export default defineConfig({
    plugins: [preact()],
    build: { // build to wwwroot so that the static files can be served easily
        outDir: "../backend/wwwroot",
        emptyOutDir: true
    },
})
