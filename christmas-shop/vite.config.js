import { defineConfig } from "vite";

export default defineConfig({
  root: ".",
  base: "/christmas-shop/",
  build: {
    outDir: "dist",
    sourcemap: true,
  },
  css: {
    devSourcemap: true,
    preprocessorOptions: {
      scss: {
        additionalData: `@use '${path.resolve(__dirname, "src/scss/variables")}' as *;`,
      },
    },
  },
});
