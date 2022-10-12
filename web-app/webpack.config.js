const CopyWebpackPlugin = require("copy-webpack-plugin");
// const MiniCssExtractPlugin = require("mini-css-extract-plugin");
// const HtmlWebpackPlugin = require('html-webpack-plugin');
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
    // new MiniCssExtractPlugin(),
    // new HtmlWebpackPlugin(),
  ],
  experiments: {
    syncWebAssembly: true,
    asyncWebAssembly: true,
  }
  // module: {
  //   rules: [
  //     {
  //       test: /\.css$/i,
  //       use: [MiniCssExtractPlugin.loader, "css-loader"],
  //     },
  //   ],
  // },
};
