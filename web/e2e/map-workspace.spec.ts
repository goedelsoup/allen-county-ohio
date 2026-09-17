import { expect, test } from '@playwright/test'

const map = '[data-map]'

async function waitForMapOutcome(page: import('@playwright/test').Page): Promise<void> {
  await expect(page.locator(map)).toHaveAttribute('data-map-ready', 'true')
  await expect(page.locator('[data-map-canvas] canvas')).toBeVisible()
  // Supplying a WebGL context can disable deck.gl's automatic drawing-buffer resize.
  // A CSS-sized 300×150 buffer still looks like a canvas but stretches and crops the county.
  await expect.poll(() => page.locator('[data-map-canvas] canvas').evaluate((element) => {
    const canvas = element as HTMLCanvasElement
    return canvas.width >= canvas.clientWidth && canvas.height >= canvas.clientHeight
  })).toBe(true)
}

async function searchLimaAndShow(page: import('@playwright/test').Page): Promise<void> {
  await waitForMapOutcome(page)
  const drawer = page.locator('.search-drawer')
  if (!(await drawer.evaluate((element) => (element as HTMLDetailsElement).open))) {
    await drawer.locator('summary').click()
  }
  const input = page.locator('[data-atlas-search-input]')
  await expect(input).toBeVisible()
  await input.fill('Lima')
  await input.press('Enter')
  const results = page.locator('[data-atlas-search-results]')
  await expect(results).toBeVisible()
  await expect(results.locator('.atlas-search-result').first()).toContainText('Lima')
  await results.locator('.atlas-search-select').first().click()
  await expect(page).toHaveURL(/[?&]at=place%2Flima.yml/)
  await expect(page.locator('[data-selected-heading]')).toHaveText('Lima')
}

test.describe('map workspace', () => {
  test.beforeEach(async ({ page }) => {
    page.on('pageerror', (error) => { throw error })
  })

  test('has no document overflow on desktop', async ({ page }) => {
    await page.goto('/')
    await expect(page.locator(map)).toBeVisible()
    await waitForMapOutcome(page)
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true)

  })

  test('has no document overflow at phone width', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 })
    await page.goto('/')
    await expect(page.locator(map)).toBeVisible()
    await waitForMapOutcome(page)
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true)
    const year = await page.locator('[data-scrub]').boundingBox()
    expect(year!.y + year!.height).toBeLessThanOrEqual(844)
    await expect(page.locator('[data-map-action="reset"]')).toBeVisible()
  })

  test('searches Lima and opens its map selection', async ({ page }) => {
    await page.goto('/')
    await searchLimaAndShow(page)
    await expect(page.locator(`${map}[data-panel-state]`)).not.toHaveAttribute('data-panel-state', 'closed')
  })

  test('keeps the selected year and era in the URL', async ({ page }) => {
    await page.goto('/')
    await waitForMapOutcome(page)
    await page.locator('details.reading > summary').click()
    await page.locator('[data-grain][value="year"]').check()
    await page.locator('[data-era]').first().click()
    const url = new URL(page.url())
    expect(Number.isInteger(Number(url.searchParams.get('year')))).toBe(true)
    expect(url.searchParams.get('grain')).toBe('year')
  })

  test('shares a view with a complete camera', async ({ page, context }) => {
    await context.grantPermissions(['clipboard-read', 'clipboard-write'], {
      origin: 'http://127.0.0.1:4321',
    })
    await page.goto('/')
    await waitForMapOutcome(page)
    await page.locator('[data-map-action="zoom-in"]').click()
    await expect.poll(() => new URL(page.url()).searchParams.has('lon')).toBe(true)
    await page.locator('[data-map-action="share"]').click()

    await expect(page.locator('[data-map-share-status]')).toContainText('Link copied')
    const shared = await page.evaluate(() => navigator.clipboard.readText())
    const sharedUrl = new URL(shared)
    expect(sharedUrl.searchParams.has('lon')).toBe(true)
    expect(sharedUrl.searchParams.has('lat')).toBe(true)
    expect(sharedUrl.searchParams.has('zoom')).toBe(true)
  })

  test('Back restores the prior selection and time', async ({ page }) => {
    await page.goto('/?year=1885&grain=year')
    await searchLimaAndShow(page)
    const selected = new URL(page.url()).searchParams
    expect(selected.get('at')).toBeTruthy()
    const selectedYear = selected.get('year')

    await page.locator('[data-era]').last().click()
    await expect.poll(() => new URL(page.url()).searchParams.get('year')).not.toBe(selectedYear)
    await page.goBack()

    await expect.poll(() => new URL(page.url()).searchParams.get('at')).toBe(selected.get('at'))
    expect(new URL(page.url()).searchParams.get('year')).toBe(selectedYear)
  })

  test('mobile inspector moves through preview, expanded, and closed states', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 })
    await page.goto('/')
    await waitForMapOutcome(page)
    const atlas = page.locator(map)
    await expect(atlas).toHaveAttribute('data-panel-state', 'closed')

    await atlas.locator('[data-panel-action="preview"]').first().click()
    await expect(atlas).toHaveAttribute('data-panel-state', 'preview')
    const sheet = await page.locator('[data-map-panel-shell]').boundingBox()
    const year = await page.locator('[data-scrub]').boundingBox()
    expect(year!.y + year!.height).toBeLessThanOrEqual(sheet!.y)
    await atlas.locator('[data-panel-action="expand"]').click()
    await expect(atlas).toHaveAttribute('data-panel-state', 'expanded')
    await atlas.locator('[data-panel-action="close"]').click()
    await expect(atlas).toHaveAttribute('data-panel-state', 'closed')
  })

  test('an article returns to the exact map context', async ({ page }) => {
    await page.goto('/?year=1885&grain=year')
    await searchLimaAndShow(page)
    const viewUrl = page.url()
    const article = page.locator('[data-map-panel] a[href^="/read/"]').first()
    await expect(article).toBeVisible()
    await article.click()
    await expect(page).toHaveURL(/\/read\//)
    await page.goBack()
    await waitForMapOutcome(page)
    await expect(page).toHaveURL(viewUrl)
    await expect(page.locator('[data-selected-heading]')).toHaveText('Lima')
  })

  test('a fresh visit restores the shared camera and selection', async ({ page, context }) => {
    await page.goto('/?year=1885&grain=year')
    await searchLimaAndShow(page)
    await page.locator('[data-map-action="zoom-in"]').click()
    const expected = new URL(page.url()).searchParams
    const fresh = await context.newPage()
    await fresh.goto(page.url())
    await waitForMapOutcome(fresh)
    const restored = new URL(fresh.url()).searchParams
    for (const key of ['lon', 'lat', 'zoom', 'year', 'grain', 'at', 'layers']) {
      expect(restored.get(key), key).toBe(expected.get(key))
    }
    await expect(fresh.locator('[data-selected-heading]')).toHaveText('Lima')
    await fresh.close()
  })

  test('an excluded event changes the year only when explicitly requested', async ({ page }) => {
    await page.goto('/?year=2026&grain=year&at=event/lima-oil-strike.yml')
    await waitForMapOutcome(page)
    await expect(page.locator('[data-selected-heading]')).toHaveText('The 1885 Lima oil strike')
    await expect(page.locator('[data-map-panel]')).toContainText('outside the selected time window')
    expect(new URL(page.url()).searchParams.get('year')).toBe('2026')
    await page.locator('[data-show-selected]').click()
    await expect.poll(() => new URL(page.url()).searchParams.get('year')).toBe('1885')
    await expect(page.locator('[data-show-selected]')).toHaveCount(0)
    await expect(page.locator('[data-selected-heading]')).toHaveText('The 1885 Lima oil strike')
  })

  test('local search and entry links survive unavailable geography', async ({ page }) => {
    await page.route('**/geo/county.geojson', (route) => route.abort())
    await page.goto('/')
    await expect(page.locator('[data-map-canvas]')).toContainText('could not be loaded')
    await page.locator('.search-drawer > summary').click()
    await page.locator('[data-atlas-search-input]').fill('Lima')
    const entry = page.locator('[data-atlas-search-results] a').first()
    await expect(entry).toHaveText('Lima')
    await entry.click()
    await expect(page).toHaveURL(/\/entry\/place\/lima/)
    await expect(page.locator('h1')).toHaveText('Lima')
  })

  test('county and records are usable while water is still loading', async ({ page }) => {
    let release!: () => void
    const delayed = new Promise<void>((resolve) => { release = resolve })
    await page.route('**/geo/linear-water.geojson', async (route) => {
      await delayed
      await route.continue()
    })
    try {
      await page.goto('/')
      await waitForMapOutcome(page)
      await expect(page.locator('[data-map-layer-status]')).toContainText('Water lines: loading')
      await searchLimaAndShow(page)
      await expect(page.locator('[data-selected-heading]')).toHaveText('Lima')
    } finally {
      release()
    }
    await expect(page.locator('[data-map-layer-status]')).toBeHidden()
  })

  test('measure geography loads on demand, recovers independently, and is cached', async ({ page }) => {
    let precinctRequests = 0
    let tractRequests = 0
    await page.route('**/geo/voting-districts.geojson', async (route) => {
      precinctRequests++
      if (precinctRequests === 1) await route.fulfill({ status: 503, body: 'Unavailable' })
      else await route.continue()
    })
    page.on('request', (request) => {
      if (request.url().endsWith('/geo/census-tracts.geojson')) tractRequests++
    })
    await page.goto('/')
    await waitForMapOutcome(page)
    await expect(page.locator('[data-map-layer-status]')).toBeHidden()
    expect(precinctRequests).toBe(0)
    expect(tractRequests).toBe(0)
    await page.locator('.layer-menu > summary').click()
    await page.locator('[data-map-measure]').selectOption('population')
    await expect(page.getByRole('button', { name: 'Retry precincts' })).toBeVisible()
    await expect(page.locator('[data-map-scale]')).toBeHidden()
    await waitForMapOutcome(page)
    await page.getByRole('button', { name: 'Retry precincts' }).click()
    await expect(page.locator('[data-map-scale]')).toBeVisible()
    await expect(page.locator('[data-map-legend]')).not.toContainText('Infinity')
    await page.locator('[data-map-measure]').selectOption('tracts')
    await expect(page.locator('[data-map-layer-status]')).toBeHidden()
    await expect(page.locator('[data-map-scale-title]')).toHaveText('People per census tract')
    await page.locator('[data-map-measure]').selectOption('population')
    await expect(page.locator('[data-map-scale-title]')).toHaveText('People per voting district')
    expect(precinctRequests).toBe(2)
    expect(tractRequests).toBe(1)
  })

  test('a late measure response respects the current layer and browser history', async ({ page }) => {
    let release!: () => void
    const delayed = new Promise<void>((resolve) => { release = resolve })
    let requests = 0
    await page.route('**/geo/voting-districts.geojson', async (route) => {
      requests++
      await delayed
      await route.continue()
    })
    await page.goto('/?layers=population')
    await waitForMapOutcome(page)
    await page.locator('.layer-menu > summary').click()
    await page.locator('[data-map-measure]').selectOption('')
    release()
    await expect(page.locator('[data-map-layer-status]')).toBeHidden()
    await expect(page.locator('[data-map-scale]')).toBeHidden()
    await page.goBack()
    await expect(page.locator('[data-map-measure]')).toHaveValue('population')
    await expect(page.locator('[data-map-scale]')).toBeVisible()
    expect(requests).toBe(1)
  })

  test('local discovery survives an unavailable WebGL context', async ({ page }) => {
    await page.addInitScript(() => {
      const original = HTMLCanvasElement.prototype.getContext
      HTMLCanvasElement.prototype.getContext = function (this: HTMLCanvasElement, type: string, ...args: unknown[]) {
        if (type.includes('webgl')) return null
        return original.call(this, type, ...args)
      } as typeof original
    })
    await page.goto('/')
    await expect(page.locator('[data-map-canvas]')).toContainText('map could not be drawn')
    await page.locator('.search-drawer > summary').click()
    await page.locator('[data-atlas-search-input]').fill('Lima')
    await page.locator('[data-atlas-search-results] a').first().click()
    await expect(page.locator('h1')).toHaveText('Lima')
  })

  for (const moveAfterSelection of [false, true]) {
    test(`pending polygon focus ${moveAfterSelection ? 'yields to later camera movement' : 'completes when its shape arrives'}`, async ({ page }) => {
      let release!: () => void
      const delayed = new Promise<void>((resolve) => { release = resolve })
      await page.route('**/geo/places.geojson', async (route) => {
        await delayed
        await route.continue()
      })
      try {
        await page.goto('/')
        await waitForMapOutcome(page)
        const initialZoom = Number(new URL(page.url()).searchParams.get('zoom'))
        await page.locator('.search-drawer > summary').click()
        await page.locator('[data-atlas-search-input]').fill('City of Lima')
        await page.locator('.atlas-search-result').filter({
          has: page.getByRole('link', { name: 'City of Lima', exact: true }),
        }).locator('.atlas-search-select').click()
        await expect(page.locator('[data-selected-heading]')).toHaveText('City of Lima')
        if (moveAfterSelection) await page.locator('[data-map-action="zoom-in"]').click()
        const beforeArrival = new URL(page.url()).searchParams
        release()
        await expect(page.locator('[data-map-layer-status]')).toBeHidden()
        const afterArrival = new URL(page.url()).searchParams
        if (moveAfterSelection) {
          for (const key of ['lon', 'lat', 'zoom']) expect(afterArrival.get(key)).toBe(beforeArrival.get(key))
        } else {
          expect(Number(afterArrival.get('zoom'))).toBeGreaterThan(initialZoom)
        }
      } finally {
        release()
      }
    })
  }
})
