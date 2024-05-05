const path = require("path");
const HTMLWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const { type } = require("os");
const { experiments } = require("webpack");

module.exports = {
  entry: "./public/main.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  experiments: { asyncWebAssembly: true },
  plugins: [
    new HTMLWebpackPlugin({ template: "./public/index.html" }),
    new WasmPackPlugin({ crateDirectory: path.resolve(__dirname, ".") }),
  ],
};
