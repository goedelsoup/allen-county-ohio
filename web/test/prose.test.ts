import { describe, expect, it } from 'vitest'
import { inline, markers, nodeName, plain, sourceName, stripMarkers, summary } from '../src/lib/prose'

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

  it('renders italic, which 117 nodes use and this did not', () => {
    // Almost all of it newspaper and ship titles, which is what italic is for.
    expect(inline('*The Times-News* (Hendersonville)')).toBe(
      '<em>The Times-News</em> (Hendersonville)',
    )
  })

  it('renders italic inside bold, which used to render as neither', () => {
    // The defect, and it is worse than not supporting italic: the old bold rule forbade a `*`
    // inside the run, so a nested one made the whole thing fail to match and the sentence
    // printed every marker it had.
    expect(inline('**The first boat was the *Marshall*, on 4 July 1845.**')).toBe(
      '<strong>The first boat was the <em>Marshall</em>, on 4 July 1845.</strong>',
    )
  })

  it('keeps two bold runs apart rather than swallowing the words between', () => {
    expect(inline('**a** and **b**')).toBe('<strong>a</strong> and <strong>b</strong>')
  })

  it('leaves a spaced asterisk alone, because that is arithmetic and not emphasis', () => {
    expect(inline('2 * 3 * 4')).toBe('2 * 3 * 4')
  })

  it('does not let a code span pair its stars with one outside it', () => {
    // The reason code is lifted out before any emphasis rule runs. Left in place, the `**`
    // inside the backticks pairs with the next one along and emphasises the closing tag.
    expect(inline('`a**b` and **c**')).toBe('<code>a**b</code> and <strong>c</strong>')
  })

  it('prints an escaped asterisk as an asterisk, not as a backslash', () => {
    // `\*` is how the corpus writes a footnote marker, and it leaked on four pages.
    expect(inline('\\* the book\'s own word')).toBe('* the book\'s own word')
    expect(inline('\\*\\* Greeley ran as the Liberal Republican')).toBe(
      '** Greeley ran as the Liberal Republican',
    )
  })

  it('leaves an escape inside a code span as written', () => {
    // Code is lifted first: a backslash inside a span is a backslash, not an escape.
    expect(inline('`a\\*b`')).toBe('<code>a\\*b</code>')
  })
})

describe('prose where markup cannot go', () => {
  it('keeps the words and drops every marker', () => {
    expect(plain('**A fact.** See [Lima](../place/lima.yml) in *The Times-News*. [verified]')).toBe(
      'A fact. See Lima in The Times-News.',
    )
  })

  it('does not let a code span pair its stars with one outside it either', () => {
    expect(plain('`a**b` and **c**')).toBe('a**b and c')
  })

  it('strips before it cuts, which is the whole of the description bug', () => {
    // `lede.text.slice(0, 180)` cut the markup and not the markdown, so a bold run longer than
    // the slice lost its closing `**` and the opening one went into a meta description.
    const long = `**${'word '.repeat(60)}end.**`
    const description = summary(long, 180)
    expect(description).not.toContain('*')
    expect(description.endsWith('…')).toBe(true)
  })

  it('returns a short block unchanged and uncut', () => {
    expect(summary('**Short.** [verified]', 180)).toBe('Short.')
  })

  it('cuts at a word boundary', () => {
    expect(summary('alpha beta gamma delta', 12)).toBe('alpha beta…')
  })
})

describe('naming a path', () => {
  it('reads a catalog path as a source name', () => {
    expect(sourceName('catalog/openelections-ohio.md')).toBe('openelections ohio')
  })

  it('reads a node id as a name', () => {
    expect(nodeName('place/allen-county.yml')).toBe('allen-county')
  })
})
