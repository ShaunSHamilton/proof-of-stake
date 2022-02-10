const MonacoWebpackPlugin = require("monaco-editor-webpack-plugin");
module.exports = function override(config, env) {
  config.plugins.push(
    new MonacoWebpackPlugin({
      // available options are documented at https://github.com/Microsoft/monaco-editor-webpack-plugin#options
      languages: ["json", "javascript", "html", "css", "typescript"],
    })
  );
  return config;
};
