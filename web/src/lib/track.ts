// The ground a moving happening covered, read off the corpus and put into a figure.
//
// Pure, and build-time or run-time indifferent: parsing and projection, no fs and no DOM. The
// county outline the track is drawn against comes from `ground.ts`, which reads `public/geo/`
// and is build-time only.
//
// ---- What a track is, and what it is not ----
//
// `event.track` is the positions a source recorded, in the order the happening passed them. For
// the two events in this corpus that carry one, that is a beginning and an end out of a federal
// storm file, and **nothing about the ground between them is recorded**. The figure draws the
// connector dashed for that reason: the endpoints are the claim, the line is the corpus's own
// reading of two points and a stated track length, and the two must not be drawn alike.
//
// So a figure over a track may reveal stated positions in order — that is progressive disclosure
// of facts the corpus holds — and may never sweep a mark along the connector between them. A
// sweep draws the storm at a position and a moment no source recorded, which is the tween
// `an-animation-asserts-continuity` refuses.

/** One recorded position, in degrees. */
export interface Point {
  lat: number
  lon: number
}

/**
 * Parse `event.track` — `"40.80, -84.20; 40.87, -83.87"`.
 *
 * Returns an empty array rather than throwing on anything malformed. A figure is furniture: a
 * page that cannot draw one should render without it, not fail to build. What must never happen
 * is a *partial* read — a track whose second point failed to parse would draw a one-point track
 * and silently lose half the claim — so a single bad pair rejects the whole value.
 *
 * ASCII hyphen only. The prose these values were lifted from writes a Unicode minus, `parseFloat`
 * does not accept it, and a lenient reader here would let one back into the corpus unnoticed.
 */
export function parseTrack(raw: string | undefined): Point[] {
  if (!raw) return []
  const points: Point[] = []
  for (const pair of raw.split(';')) {
    const [rawLat, rawLon, ...rest] = pair.split(',')
    if (rest.length > 0 || rawLon === undefined) return []
    const lat = Number(rawLat.trim())
    const lon = Number(rawLon.trim())
    if (!Number.isFinite(lat) || !Number.isFinite(lon)) return []
    if (lat < -90 || lat > 90 || lon < -180 || lon > 180) return []
    points.push({ lat, lon })
  }
  return points.length >= 2 ? points : []
}

/**
 * Web Mercator, in radians of longitude and its `y` counterpart.
 *
 * The same projection the map uses, so a track and the map do not disagree about the shape of
 * the county. At this county's latitude an equirectangular plot would be about a per cent out
 * over twenty miles, which is invisible — but the two figures sit on one site and being right
 * for a reason is cheaper than being close for none.
 */
export function project({ lat, lon }: Point): { x: number; y: number } {
  const clamped = Math.max(Math.min(lat, 85), -85)
  return {
    x: (lon * Math.PI) / 180,
    y: Math.log(Math.tan(Math.PI / 4 + (clamped * Math.PI) / 360)),
  }
}

/** A projected extent, and the SVG box it is drawn into. */
export interface Frame {
  width: number
  height: number
  /** Projected → SVG. */
  place: (p: Point) => { x: number; y: number }
}

/**
 * Fit an extent into a box of a given width, preserving shape.
 *
 * The height falls out of the county's own aspect ratio rather than being chosen: a figure that
 * fixed both dimensions would stretch the county, and a county drawn wrong is worse than no
 * county. `pad` is in SVG units and keeps a mark on the boundary from being clipped in half.
 */
export function frame(extent: Point[], width: number, pad = 6): Frame {
  const projected = extent.map(project)
  const xs = projected.map((p) => p.x)
  const ys = projected.map((p) => p.y)
  const minX = Math.min(...xs)
  const maxX = Math.max(...xs)
  const minY = Math.min(...ys)
  const maxY = Math.max(...ys)
  const spanX = maxX - minX || 1
  const spanY = maxY - minY || 1
  const inner = width - pad * 2
  const scale = inner / spanX
  const height = spanY * scale + pad * 2

  return {
    width,
    height,
    place: (p: Point) => {
      const { x, y } = project(p)
      return {
        x: pad + (x - minX) * scale,
        // SVG y grows downward and Mercator y grows north, so the axis is flipped here rather
        // than by a transform on the element — a transform would flip the labels too.
        y: pad + (maxY - y) * scale,
      }
    },
  }
}

/** `40.87, -83.87` → `40.87° N, 83.87° W`, for a caption that reads as English. */
export function bearing({ lat, lon }: Point): string {
  const ns = lat >= 0 ? 'N' : 'S'
  const ew = lon >= 0 ? 'E' : 'W'
  return `${Math.abs(lat).toFixed(2)}° ${ns}, ${Math.abs(lon).toFixed(2)}° ${ew}`
}

const rad = (degrees: number): number => (degrees * Math.PI) / 180

/** Metres between two positions on a sphere. */
export function metres(a: Point, b: Point): number {
  const R = 6_371_008.8
  const dLat = rad(b.lat - a.lat)
  const dLon = rad(b.lon - a.lon)
  const h =
    Math.sin(dLat / 2) ** 2 + Math.cos(rad(a.lat)) * Math.cos(rad(b.lat)) * Math.sin(dLon / 2) ** 2
  return 2 * R * Math.asin(Math.sqrt(h))
}

/**
 * The straight-line length of a track, in miles.
 *
 * **Not the length of the happening.** The source states a track length of its own — 17.8 miles
 * for the 1965 tornado — and this is the distance between the positions it recorded, which came
 * out at 17.92. The corpus keeps both and reads the near-agreement as identifying the storm
 * rather than as measuring it. A figure printing this number must say which of the two it is.
 */
export function straightMiles(points: Point[]): number {
  let total = 0
  for (let i = 1; i < points.length; i++) total += metres(points[i - 1], points[i])
  return total / 1609.344
}
