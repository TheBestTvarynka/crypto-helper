const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./src/bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin({ patterns: [ { from: 'src', to: '' }] }),
  ],
  experiments: {
    syncWebAssembly: true,
    asyncWebAssembly: true,
  }
};
