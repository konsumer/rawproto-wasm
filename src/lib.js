// this is the entry-point for the bundler

import r from '../target/web/rawproto_bg.js'

export const parseRaw = r.parse_raw

export function getString (binary, path) {
  const entries = parseRaw(binary, path, { no_fixed64: true, no_fixed32: true })
  return entries.map(entry => {
    return String.fromCharCode(...entry.value.Bytes)
  })
}
