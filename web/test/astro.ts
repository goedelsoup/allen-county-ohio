// Splitting an `.astro` file into its two halves, without guessing.
//
// Three checks used to do this with `source.split('---')`, and that is wrong in a way nothing
// reports. An Astro file's frontmatter is delimited by a `---` **on its own line**, and a `---`
// appearing anywhere inside it — in a comment rule, in a markdown table, in a string — splits the
// file somewhere else. The frontmatter then reads short, the body reads long, and every check
// built on the split quietly stops covering the page.
//
// It happened here. A section separator written as `// ---- the county in four numbers -------`
// in the rearranged index took two assertions out of `coverage.test.ts`'s reckoning, and the only
// reason it was noticed is that those two assertions had nowhere else to be rendered. Anything
// that had a second home would have gone unmissed.

export interface Parts {
  /** The script between the delimiters, or `''` on a file that has no frontmatter. */
  frontmatter: string
  /** Everything after the closing delimiter. The whole file, where there is no frontmatter. */
  body: string
  /** Whether the file opened with a frontmatter block at all. */
  fenced: boolean
}

/** The delimiter, and only when it is the whole line. */
const FENCE = /^-{3,}\s*$/

export function parts(source: string): Parts {
  const lines = source.split('\n')
  if (!FENCE.test(lines[0] ?? '')) return { frontmatter: '', body: source, fenced: false }

  const close = lines.findIndex((line, i) => i > 0 && FENCE.test(line))
  if (close === -1) return { frontmatter: '', body: source, fenced: false }

  return {
    frontmatter: lines.slice(1, close).join('\n'),
    body: lines.slice(close + 1).join('\n'),
    fenced: true,
  }
}

/** The script half. */
export const frontmatter = (source: string): string => parts(source).frontmatter

/** The markup half — which is what every prose and structure check is actually about. */
export const body = (source: string): string => parts(source).body
