## Install

```bash
npm install
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## Build

```bash
wasm-pack -V

# https://rustwasm.github.io/docs/wasm-pack/commands/build.html
#
# Outputs JS that can be natively imported as an ES module in a
# browser, but the WebAssembly must be manually instantiated
# and loaded.
wasm-pack build --target web --dev

# or...
npm run build
```

# Serve

```bash
# start hotel (optional)
hotel start
# ensure the app is a hotel host
npm run hotelize
npm start # or through the hotel interface at http://hotel.loc
# go to http://rustweb.loc
```
