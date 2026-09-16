import { searchAtlas, type AtlasSearchItem } from '../lib/map-search'

/** Kept small so the renderer has one safe route for feed-owned strings. */
export function putText(target: { textContent: string | null }, value: string): void {
  target.textContent = value
}

const element = <K extends keyof HTMLElementTagNameMap>(
  name: K,
  className?: string,
): HTMLElementTagNameMap[K] => {
  const made = document.createElement(name)
  if (className) made.className = className
  return made
}

const resultRow = (root: HTMLElement, item: AtlasSearchItem): HTMLLIElement => {
  const row = element('li', 'atlas-search-result')
  const body = element('span', 'atlas-search-result-body')
  const link = element('a', 'atlas-search-result-link')
  link.setAttribute('href', item.href)
  putText(link, item.label)
  body.append(link)

  const meta = element('span', 'atlas-search-result-meta')
  const context = element('span')
  putText(context, item.context)
  const when = element('span')
  putText(when, item.when)
  meta.append(context, when)
  body.append(meta)
  row.append(body)

  if (item.node) {
    const node = item.node
    const select = element('button', 'atlas-search-select')
    select.type = 'button'
    putText(select, 'Show on map')
    select.addEventListener('click', () => {
      root.dispatchEvent(
        new CustomEvent('atlas:select', {
          bubbles: true,
          detail: { node },
        }),
      )
    })
    row.append(select)
  }

  return row
}

/** Attach local search without depending on the map renderer or WebGL. */
export function initAtlasSearch(root: HTMLElement): void {
  const source = root.querySelector<HTMLScriptElement>('[data-atlas-search-index]')
  const form = root.querySelector<HTMLFormElement>('[data-atlas-search-form]')
  const input = root.querySelector<HTMLInputElement>('[data-atlas-search-input]')
  const clear = root.querySelector<HTMLButtonElement>('[data-atlas-search-clear]')
  const summary = root.querySelector<HTMLElement>('[data-atlas-search-summary]')
  const results = root.querySelector<HTMLOListElement>('[data-atlas-search-results]')
  if (!source?.textContent || !form || !input || !clear || !summary || !results) return

  let index: AtlasSearchItem[]
  try {
    index = JSON.parse(source.textContent) as AtlasSearchItem[]
  } catch {
    // A malformed embedded index is an unavailable instrument, so use the site's status ink
    // as well as saying so. The text remains the signal; colour is only reinforcement.
    summary.style.color = getComputedStyle(document.documentElement)
      .getPropertyValue('--status-open-text')
      .trim()
    putText(summary, 'Search is unavailable.')
    return
  }

  const reset = () => {
    results.replaceChildren()
    results.hidden = true
    clear.hidden = true
    putText(summary, `Search ${index.length.toLocaleString('en-US')} atlas records and articles.`)
  }

  const run = () => {
    const query = input.value.trim()
    if (!query) {
      reset()
      return
    }

    const found = searchAtlas(index, query)
    results.replaceChildren(...found.map((item) => resultRow(root, item)))
    results.hidden = found.length === 0
    clear.hidden = false
    putText(
      summary,
      found.length === 0
        ? `No results for \u201c${query}\u201d.`
        : `${found.length} result${found.length === 1 ? '' : 's'} for \u201c${query}\u201d.`,
    )
  }

  form.addEventListener('submit', (event) => {
    event.preventDefault()
    run()
  })
  // The index is small and local, so typing can update it without a request or a loading state.
  input.addEventListener('input', run)
  clear.addEventListener('click', () => {
    input.value = ''
    reset()
    input.focus()
  })

  reset()
}
