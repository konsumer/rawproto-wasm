// this is the library entry-point

import init from '../target/web/rawproto.js'
import wasmBytes from '../target/web/wasm.js'

let r

// wrap parse_raw with optional params
export function parseRaw(bin, path='.', cfg = { no_fixed64: false, no_fixed32: false }) {
  if (!r) {
     r = await init(wasmBytes)
  }
  return r.parse_raw(bin, path, cfg)
}

// get a string from a path
export async function getString (binary, path) {
  const entries = parseRaw(binary, path)
  return entries.map(entry => {
    return String.fromCharCode(...entry.value.Bytes)
  })
}
