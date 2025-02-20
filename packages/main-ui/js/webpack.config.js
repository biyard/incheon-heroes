const webpack = require("webpack");
const path = require("path");

module.exports = {
  output: {
    globalObject: "this",
    filename: "dep.js",
    path: path.resolve(__dirname, "dist"),
  },
  entry: "./src/index.ts",
  devtool: "inline-source-map",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: [
          /node_modules/,
        ]
      },
    ],
  },
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true,
  },
  resolve: {
    fallback: {
      fs: false,
      net: false,
      url: false,
      stream: require.resolve("stream-browserify"),
      crypto: require.resolve("crypto-browserify"),
      http: require.resolve("stream-http"),
      https: require.resolve("https-browserify"),
      os: require.resolve("os-browserify/browser"),
      constants: require.resolve("constants-browserify"),
      vm: require.resolve("vm-browserify"),
      buffer: require.resolve("buffer"),
    },
    extensions: [".tsx", ".ts", ".js"],
  },
  plugins: [
    new webpack.ProvidePlugin({
      process: "process/browser",
    }),
    new webpack.ProvidePlugin({
      Buffer: ["buffer", "Buffer"],
    }),
  ],
};
