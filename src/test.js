const { parse_raw } = require('../pkg/rawproto.js')

function getString (binary, path) {
  const entries = parse_raw(binary, path, { no_fixed64: true, no_fixed32: true })
  return entries.map(entry => {
    return String.fromCharCode(...entry.value.Bytes)
  })
}

describe('rawproto', () => {
  test('basic parsing', () => {
    const opt = { no_fixed64: true, no_fixed32: false }
    expect(parse_raw(Buffer.from('\x12\x07Unknown'), '', opt)).toMatchSnapshot()
    expect(parse_raw(Buffer.from('\x12\x07Unknown\x12\x07Unknown'), '', opt)).toMatchSnapshot()
  })

  test('string parsing', () => {
    expect(getString(Buffer.from('\x12\x07Unknown\x12\x07Unknown'), '.2')).toMatchSnapshot()
  })
})
