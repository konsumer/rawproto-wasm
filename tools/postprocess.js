// this will generate an inline-able wasm object & clean up wasm-pack output

import { readFile, writeFile } from 'fs/promises'

const bytes = await readFile('target/web/rawproto_bg.wasm')

writeFile('target/web/wasm.js', `export default Uint8Array.from(${JSON.stringify(Array.from(bytes), null, 2)})`)
