// this is the library entry-point

import init from '../target/web/rawproto.js'
import wasmBytes from '../target/web/wasm.js'

let r

async function sideeffect () {
   r = await init(wasmBytes)
}
sideeffect()

// wrap parse_raw with optional params
export function parseRaw(bin, path='.', cfg = { no_fixed64: false, no_fixed32: false }) {
  return r.parse_raw(bin, path, cfg)
}

// get a string from a path
export function getString (binary, path) {
  const entries = parseRaw(binary, path)
  return entries.map(entry => {
    return String.fromCharCode(...entry.value.Bytes)
  })
}
