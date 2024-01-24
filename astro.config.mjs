import { defineConfig } from 'astro/config';
import sentry from "@sentry/astro";
import spotlightjs from "@spotlightjs/astro";

import tailwind from "@astrojs/tailwind";

// https://astro.build/config
export default defineConfig({
  integrations: [sentry(), spotlightjs(), tailwind()],
  vite: {
    server: {
      port: 4321,
      watch: {
        ignored: ['**/target/**', '**/src-tauri/**']
      }
    }
  }
});