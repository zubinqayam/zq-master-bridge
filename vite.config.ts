import { realpathSync } from "node:fs";
import { resolve } from "node:path";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

const workspaceRoot = realpathSync(resolve("."));

export default defineConfig({
  root: workspaceRoot,
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: ["es2021", "chrome100", "safari13"],
    minify: process.env.TAURI_DEBUG !== "1" ? "esbuild" : false,
    sourcemap: process.env.TAURI_DEBUG === "1",
    rollupOptions: {
      input: resolve(workspaceRoot, "index.html"),
    },
  },
});
