const { parse_raw } = require('../pkg/rawproto.js')

test('rawproto', () => {
  const opt = { no_fixed64: true, no_fixed32: false }
  expect(parse_raw(Buffer.from("\x12\x07Unknown"), '', opt)).toMatchSnapshot()
  expect(parse_raw(Buffer.from("\x12\x07Unknown\x12\x07Unknown"), '', opt)).toMatchSnapshot()
})
