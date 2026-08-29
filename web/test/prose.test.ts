import { describe, expect, it } from 'vitest'
import { inline, markers, nodeName, sourceName, stripMarkers } from '../src/lib/prose'

describe('rendering corpus prose', () => {
  it('promotes the claim tag out of the sentence', () => {
    // The tag is not deleted — it becomes a badge. Leaving it inline says it twice.
    expect(stripMarkers('The county holds 402.545 square miles. [verified]')).toBe(
      'The county holds 402.545 square miles.',
    )
  })

  it('reads every tag in a block, in order', () => {
    expect(markers('Solid. [verified] Softer. [inference]')).toEqual(['verified', 'inference'])
  })

  it('renders a corpus link as its words, not as an anchor to nowhere', () => {
    // Corpus links point at `.yml` files that do not exist on this site.
    expect(inline('see [Shawnee Township](../place/shawnee-township.yml)')).toBe(
      'see Shawnee Township',
    )
  })

  it('resolves a link when the caller knows where it goes', () => {
    const html = inline('see [Lima](../place/lima.yml)', (t) =>
      t.includes('lima') ? '/places/lima' : undefined,
    )
    expect(html).toBe('see <a href="/places/lima">Lima</a>')
  })

  it('escapes markup before introducing any of its own', () => {
    expect(inline('a <script>alert(1)</script> b')).toBe(
      'a &lt;script&gt;alert(1)&lt;/script&gt; b',
    )
  })

  it('renders bold and inline code together on one line', () => {
    // The corpus writes exactly this: `MUNI` **blank**.
    expect(inline('`MUNI` **blank**')).toBe('<code>MUNI</code> <strong>blank</strong>')
  })

  it('does not treat a bold marker inside a code span as emphasis', () => {
    expect(inline('`a**b`')).toBe('<code>a**b</code>')
  })

  it('reads a catalog path as a source name', () => {
    expect(sourceName('catalog/openelections-ohio.md')).toBe('openelections ohio')
  })

  it('reads a node id as a name', () => {
    expect(nodeName('place/allen-county.yml')).toBe('allen-county')
  })
})
