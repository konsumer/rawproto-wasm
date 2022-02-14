# rawproto-wasm

Decode protobuf in raw form, in wasm (for node, deno, cf workers, etc.) Based on [decode_raw](https://github.com/confio/decode_raw), which is a rad CLI tool for looking at binary protobuf.

You can see a web-demo [here](https://rawprotorust.surge.sh/)

This has some ideas in common with [rawproto](https://github.com/konsumer/rawproto), but marks a different direction that can parse a bit faster, and eventually handle partial parsing (mixing proto def with raw fields.)

## dev

```
# build the module
npm run build 

# start local test-server
npm start
```