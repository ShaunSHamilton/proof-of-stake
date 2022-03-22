import { wasm } from "@rollup/plugin-wasm";

export default {
  input: "node/index.js",
  plugins: [wasm()],
};
