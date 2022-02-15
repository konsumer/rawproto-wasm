/* global describe, test, expect */
import { parseRaw, getString } from './lib.js'

describe('rawproto', () => {
  test('basic parsing', () => {
    const opt = { no_fixed64: true, no_fixed32: false }
    expect(parseRaw(Buffer.from('\x12\x07Unknown'), '', opt)).toMatchSnapshot()
    expect(parseRaw(Buffer.from('\x12\x07Unknown\x12\x07Unknown'), '', opt)).toMatchSnapshot()
  })

  test('string parsing', () => {
    expect(getString(Buffer.from('\x12\x07Unknown\x12\x07Unknown'), '.2')).toMatchSnapshot()
  })
})
