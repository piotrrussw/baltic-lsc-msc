import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: { port: 3009 },
  build: {
    minify: false,
    // outDir: "dist/client",
    // ssrManifest: true, // Ensure this option is set
  },
});