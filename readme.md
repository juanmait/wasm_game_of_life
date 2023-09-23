# wasm_game_of_life

This is basically the game of life written in Rust and compiled to web assembly.

**IMPORTANT**: This repo is old and have many outdated nodejs dependencies that are currently
flagged as vulnerabilities by Github.

## Install

```bash
# install npm dependencies
npm install

# install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Or install (and build) from source..
cargo install wasm-pack
```

## Build

````bash
# check wasm-pack installation
wasm-pack -V

# build with webpack
npm run build

# in new node versions you can get an error like this:
#
# ```
# node:internal/crypto/hash:67
#   this[kHandle] = new _Hash(algorithm, xofLen);
# ```
# the node option `--openssl-legacy-provider` solved that in my case
NODE_OPTIONS=--openssl-legacy-provider npm run build
````

# Serve

````bash
# start the dev server and compiler
npm start

# in new node versions you can get an error like this:
#
# ```
# node:internal/crypto/hash:67
#   this[kHandle] = new _Hash(algorithm, xofLen);
# ```
# the node option `--openssl-legacy-provider` solved that in my case
NODE_OPTIONS=--openssl-legacy-provider npm start
````
