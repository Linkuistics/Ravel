import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

// The page loads the example grammars from `../examples`, outside the root of
// the page. The dev server serves files outside its root only from the
// directories in `server.fs.allow` (https://vite.dev/config/server-options).
const repository = fileURLToPath(new URL("..", import.meta.url));

export default defineConfig({
  server: { fs: { allow: [repository] } },
  build: { target: "es2024" },
});
