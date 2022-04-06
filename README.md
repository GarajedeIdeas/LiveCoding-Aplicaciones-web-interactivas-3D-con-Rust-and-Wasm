# Garaje Live Coding (2022)

- [glc-2022](#glc-2022)
  - [0. Environment setup](#0-environment-setup)
  - [1. Project setup](#1-project-setup)

## 0. Environment setup
Install Rust [https://www.rust-lang.org/tools/install]
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Make sure it’s up to date
```
rustup update
```

Install wasm32 target
```
rustup target install wasm32-unknown-unknown
```

Install Rust formatter
```
rustup component add rustfmt
```

Install Rust Clippy
```
rustup component add clippy
```

Install VS Code [https://code.visualstudio.com/]

Install rust-analyzer [https://rust-analyzer.github.io/]

Install Node.js [https://nodejs.org/en/download/package-manager/#macos]
```
brew update
brew install node
```

Install wasm-pack
```
sudo npm i -g wasm-pack –unsafe-perm=true
```

## 1. Project setup

$PROJECT_ROOT is the root folder of the project
```
export PROJECT_ROOT=~/Documents/GarajeLiveCoding/
mkdir $PROJECT_ROOT
cd $PROJECT_ROOT
```

Create Rust library
```
cd $PROJECT_ROOT
wasm-pack new glc-rs
cd glc-rs
rm -rf .git
```

Try cargo commands
```
cargo build
cargo fmt
cargo clippy
```

Do a proper build
```
wasm-pack build
```

Create react app with Webpack5 and React17 and Babel [https://dev.to/riyanegi/setting-up-webpack-5-with-react-and-babel-from-scratch-2021-271l]
```
cd $PROJECT_ROOT
mkdir glc-ui
npm init -y
npm i react@17.0.2 react-dom@17.0.2
npm i -D @babel/core @babel/preset-env @babel/preset-react babel-loader file-loader css-loader style-loader webpack webpack-cli html-webpack-plugin
```

Create Babel configuration file
```
touch .babelrc
```

Add these lines
```json
{
    "presets": [
         "@babel/preset-env",
         "@babel/preset-react"
    ]
}
```

Create Webpack configuration file
```
touch webpack.config.js
```

Add these lines
```js
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  entry: './src/index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.js'
  },
  devServer: {
    port: 3000,
    watchFiles: ['src/**/*', '../glc-rs/pkg/*'] 
  },
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader'
        }
      },
      {
        test: /\.css$/,
        use: ['style-loader', 'css-loader']
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({ template: './src/index.html' })
  ],
  experiments: {
    asyncWebAssembly: true
  }
};
```

Add our wasm package to our dependencies inside package,json
```json
{
  "devDependencies": {
    "glc-wasm": "file:../glc-rs/pkg"
  }
}
```

Make sure to install the new dependency
```
npm install
```

Add some more handy dependencies
```
npm i --save @mui/material @mui/icons-material @emotion/react @emotion/styled react-dropzone
```

Setup basic react file structure
```
mkdir src
touch src/index.js src/index.html src/App.js
```

Add this to index.html
```html
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="initial-scale=1, width=device-width" />
        <title>Garaje Live Coding</title>
    </head>
    <body>
        <div id="app"></div>
        <noscript>This page contains webassembly and javascript content, please enable javascript in your browser.</noscript>
    </body>
</html>
```

Add this to index.js
```js
import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';

ReactDOM.render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
    document.getElementById('app')
);
```

Add this to App.js
```js
import React from "react";
import CssBaseline from '@mui/material/CssBaseline';
import { Container } from "@mui/material";
import { greet } from 'glc-wasm';

greet();

export default function App() {
    return (
        <React.Fragment>
            <CssBaseline>
                <Container>
                    Hello World!
                </Container>
            </CssBaseline>
        </React.Fragment>
    );
}
```

Add scripts to package.json
```json
{
    "scripts": {
        "serve": "webpack serve --mode development",
        "build": "webpack --mode production" 
    },
}
```

Install Webpack dev server
```
npm install -D webpack-dev-server
```

Serve from localhost
```
npm run serve
```

The setup is done and by navigating to http://localhost:3000 you should get an alert fired by the rust-wasm library.

