const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
  module: {
    rules: [
      {
        test:/\.(js|jsx)$/,
        include: path.resolve(__dirname, 'src'),
        loader: 'babel-loader',
        query: 
        {
            presets: ['es2015','react']
        }
      },
      {
        test: /\.css$/,  
        include: path.resolve(__dirname, 'src'),
        loaders: ['style-loader', 'css-loader'],
      }
    ]
 }
};
