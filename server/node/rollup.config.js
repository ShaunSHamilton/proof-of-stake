import replace from "@rollup/plugin-replace";

export default {
  input: "client/index.js",
  output: {
    file: "client/bundle.js",
    format: "iife",
  },
  plugins: [
    replace({
      "process.env.LOG_LEVEL": "'debug'",
      delimiters: ["", ""],
      preventAssignment: true,
    }),
  ],
};
