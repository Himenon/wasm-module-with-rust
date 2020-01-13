// @ts-check
const webpack = require("webpack");
const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

/**
 * @returns {webpack.Configuration}
 */
const generateConfig = () => {
  return {     
    entry: "./index.js",
    output: {         
        path: path.resolve(__dirname, "dist"),         
        filename: "bundle.js",     
    },
    plugins: [         
        new HtmlWebpackPlugin(),         
    ],
    mode: "development"
  };
}

module.exports = generateConfig();
