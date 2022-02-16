/* global describe, test, expect */
import { parseRaw, getString } from '../src/lib.js'

const test1 = Buffer.from('\x12\x07Unknown')
const test2 = Buffer.from('\x12\x07Unknown\x12\x07Unknown')

describe('rawproto', () => {
  test('basic parsing', () => {
    expect(parseRaw(test1)).toMatchSnapshot()
    expect(parseRaw(test2)).toMatchSnapshot()
  })

  test('string parsing', () => {
    expect(getString(test2, '.2')).toMatchSnapshot()
  })
})
