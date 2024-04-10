import { defineConfig } from "astro/config";
import wasm from "vite-plugin-wasm";
import react from "@astrojs/react";

// https://astro.build/config
export default defineConfig({
  srcDir: "frontend",
  vite: {
    optimizeDeps: {
      entries: ["layouts", "pages", "components"],
    },
    plugins: [wasm()],
  },
  integrations: [react()],
});
