import assert from 'node:assert/strict'
import palate from '../index.js'

assert.ok(palate.version())
assert.equal(palate.detect('main.rs', Buffer.from('fn main() {}\n')), 'rust')
assert.equal(palate.tryDetect('unknown.file', Buffer.from('')), null)
assert.equal(palate.detect('main.rs', new Uint8Array(Buffer.from('fn main() {}\n'))), 'rust')
assert.equal(palate.detect('main.rs', Buffer.from('fn main() {\0}\n')), 'rust')

console.log('palate node smoke tests passed')
