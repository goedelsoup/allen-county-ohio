import { describe, expect, it } from 'vitest'
import { classes, classSchema, nodes } from '../src/lib/feeds'
import {
  aliasesFor,
  className,
  entryPath,
  entrySlug,
  holdingsFor,
  keysFor,
  nodesOfClass,
  parseCoords,
  propertyLabel,
  relationsFor,
  resolveCorpusLink,
  schemaRows,
  sourcesFor,
  year,
} from '../src/lib/entry'

describe('entry paths', () => {
  it('drops the extension a corpus id carries', () => {
    expect(entryPath('tenure/sheriff-1931-jess-l-sarber.yml')).toBe(
      '/entry/tenure/sheriff-1931-jess-l-sarber',
    )
    expect(entrySlug('place/lima.yml')).toBe('place/lima')
  })
})

describe('resolving a corpus link', () => {
  it('resolves a link to a node this site publishes', () => {
    const published = nodes.find((n) => n.class === 'place')!
    const name = published.id.replace('place/', '').replace('.yml', '')
    expect(resolveCorpusLink(`../place/${name}.yml`)).toBe(`/entry/place/${name}`)
  })

  it('refuses a catalog source, which has no page here', () => {
    // The corpus cites catalog files the same way it links nodes. An anchor to a page that
    // does not exist is worse than plain words, so these stay unresolved.
    expect(resolveCorpusLink('../../catalog/census-gazetteer-2020.md')).toBeUndefined()
  })

  it('refuses a node the feed does not carry', () => {
    // A node can be withheld by the publication rule while still being linked from prose
    // that was published. The link has to fail closed.
    expect(resolveCorpusLink('../place/a-place-that-was-never-written.yml')).toBeUndefined()
  })

  it('refuses a target that is not a corpus class', () => {
    expect(resolveCorpusLink('../notaclass/whatever.yml')).toBeUndefined()
  })
})

describe('sources behind an entry', () => {
  it('collects catalog citations in first-appearance order, without repeats', () => {
    const cited = nodes.find((n) => sourcesFor(n).length > 1)!
    const sources = sourcesFor(cited)
    expect(sources).toEqual([...new Set(sources)])
    expect(sources.every((s) => s.startsWith('catalog/'))).toBe(true)
  })
})

describe('relations', () => {
  it('carries edges in both directions, each knowing which way it points', () => {
    const sheriff = 'office/allen-county-sheriff.yml'
    const relations = relationsFor(sheriff)
    expect(relations.length).toBeGreaterThan(0)
    expect(relations.some((r) => r.direction === 'in')).toBe(true)
    for (const r of relations) {
      const near = r.direction === 'out' ? r.edge.from : r.edge.to
      expect(near).toBe(sheriff)
    }
  })

  it('keeps an edge whose far end is unpublished, rather than dropping the claim', () => {
    // The relationship was still asserted. Hiding it would make the graph look tidier than
    // it is; the row renders without a link instead.
    for (const r of relationsFor('office/allen-county-sheriff.yml')) {
      expect(r.otherId).toBeTruthy()
    }
  })
})

describe('reading the feed without disturbing it', () => {
  it('does not reorder the shared node list when grouping by class', () => {
    // `nodes` is a module-level array every page shares. Sorting it in place would change
    // what another page renders depending on which one built first.
    const before = nodes.map((n) => n.id)
    nodesOfClass('tenure')
    expect(nodes.map((n) => n.id)).toEqual(before)
  })
})

describe('labels', () => {
  it('reads a snake-case property key as a phrase', () => {
    expect(propertyLabel('how_began')).toBe('How began')
  })

  it('reads a hyphenated class name as a phrase', () => {
    expect(className('natural-feature')).toBe('Natural feature')
  })
})

describe('holdings, from each corner', () => {
  const SHERIFF = 'office/allen-county-sheriff.yml'

  it('lists an office’s holdings oldest first', () => {
    const office = nodes.find((n) => n.id === SHERIFF)!
    const holdings = holdingsFor(office, 'holders')
    expect(holdings.length).toBeGreaterThan(30)
    const spans = holdings.map((h) => h.began)
    expect(spans).toEqual(spans.toSorted())
    expect(holdings.every((h) => h.office?.id === SHERIFF)).toBe(true)
  })

  it('gives a tenure the whole succession of its seat, itself included', () => {
    const office = nodes.find((n) => n.id === SHERIFF)!
    const oneTenure = holdingsFor(office, 'holders')[0].tenure
    const seat = holdingsFor(oneTenure, 'seat')
    expect(seat.map((h) => h.tenure.id)).toContain(oneTenure.id)
    expect(seat.length).toBe(holdingsFor(office, 'holders').length)
  })

  it('gives a person the offices they held', () => {
    const office = nodes.find((n) => n.id === SHERIFF)!
    const person = holdingsFor(office, 'holders').find((h) => h.person)!.person!
    const held = holdingsFor(person, 'offices')
    expect(held.length).toBeGreaterThan(0)
    expect(held.every((h) => h.person?.id === person.id)).toBe(true)
  })

  it('returns nothing for a class that has no holdings', () => {
    const place = nodes.find((n) => n.class === 'place')!
    expect(holdingsFor(place, 'seat')).toEqual([])
    expect(holdingsFor(place, 'offices')).toEqual([])
  })
})

describe('coordinates', () => {
  it('reads a bare pair', () => {
    expect(parseCoords('40.771627, -84.106103')).toEqual({ lat: 40.771627, lon: -84.106103 })
  })

  it('reads a pair that has a sentence and a claim tag after it', () => {
    // `natural-feature.mouth` is written this way: the numbers, then the county,
    // then what it joins, then the tag. Only the head is a coordinate.
    const parsed = parseCoords(
      '41.2875489, -84.3557802, in Defiance County. Joins the Maumee River. [inference]',
    )
    expect(parsed).toEqual({ lat: 41.2875489, lon: -84.3557802 })
  })

  it('refuses prose that does not start with a pair', () => {
    expect(parseCoords('Rises in Roundhead Township')).toBeUndefined()
    expect(parseCoords(undefined)).toBeUndefined()
  })
})

describe('external keys and aliases', () => {
  it('picks out the identifier fields and labels them', () => {
    const place = nodes.find((n) => n.class === 'place' && n.properties.gnis_id)!
    const keys = keysFor(place)
    expect(keys.map((k) => k.label)).toContain('GNIS')
    expect(keys.every((k) => k.value.length > 0)).toBe(true)
  })

  it('splits an alias field written as one comma-joined string', () => {
    const org = nodes.find((n) => n.properties.also_known_as)!
    const aliases = aliasesFor(org)
    expect(aliases.length).toBeGreaterThan(0)
    expect(aliases.every((a) => a === a.trim() && a.length > 0)).toBe(true)
  })

  it('gives nothing where the node carries no aliases', () => {
    const bare = nodes.find((n) => !n.properties.also_known_as && !n.properties.former_names)!
    expect(aliasesFor(bare)).toEqual([])
  })
})

describe('years', () => {
  it('takes the four-digit year out of whatever precision the corpus wrote', () => {
    expect(year('1888-11-06')).toBe(1888)
    expect(year('1885')).toBe(1885)
    expect(year('c. 1903')).toBe(1903)
    expect(year('')).toBeNull()
  })
})

describe('the schema card', () => {
  it('has a declaration for every class the site renders a page for', () => {
    // The card is the one piece of furniture common to all thirteen classes. A class with
    // nodes but no declaration would render a card with holes in it, and the only place that
    // would show is the page itself.
    const missing = [...new Set(nodes.map((n) => n.class))].filter((c) => !classSchema(c))
    expect(missing).toEqual([])
  })

  it('says what the corpus declares, not what this file thinks', () => {
    // The whole reason the ontology crossed the feed boundary. If these ever have to be
    // edited to match a corpus change, the copy has drifted and the feed is doing nothing.
    expect(classSchema('tenure')?.foundational_type).toBe('relator')
    expect(classSchema('office')?.foundational_type).toBe('role')
    expect(classSchema('question')?.foundational_type).toBe('situation')
    expect(classSchema('measure')?.required).toEqual(['parameter', 'value', 'as_of'])
    expect(classSchema('person')?.required).toEqual([])
  })

  it('gives four rows, the last of them class-specific', () => {
    const tenure = schemaRows('tenure')
    expect(tenure.map((r) => r.label)).toEqual(['Class', 'Ontology', 'Foundational', 'Edge policy'])

    // A class that requires properties spends the fourth row on them instead. Edge policy is
    // `characteristic` for all thirteen, so it is what the card says when it has nothing
    // sharper to say.
    const measure = schemaRows('measure')
    expect(measure.map((r) => r.label)).toEqual(['Class', 'Ontology', 'Foundational', 'Required'])
    expect(measure[3].value).toBe('parameter · value · as_of')
  })

  it('accents at most one row, and only where the class departs from the default', () => {
    for (const cls of Object.keys(classes)) {
      const accented = schemaRows(cls).filter((r) => r.accent)
      expect(accented.length, `${cls} accents ${accented.length} rows`).toBeLessThanOrEqual(1)
    }

    // A relator is an argument for a class; a kind is not, and its card stays quiet.
    expect(schemaRows('tenure').find((r) => r.accent)?.value).toBe('relator')
    expect(schemaRows('person').some((r) => r.accent)).toBe(false)

    // Where a class both requires properties and is not a kind, the requirement takes it —
    // one point per card, and the requirement is the sharper one.
    expect(schemaRows('measure').find((r) => r.accent)?.label).toBe('Required')
    expect(schemaRows('measure').find((r) => r.label === 'Foundational')?.accent).toBe(false)
  })

  it('carries the class’s own account of itself, unwrapped', () => {
    for (const [cls, schema] of Object.entries(classes)) {
      expect(schema.description.length, `${cls} says nothing about why it exists`).toBeGreaterThan(0)
      expect(schema.description, `${cls} reached the feed still wrapped`).not.toContain('\n')
    }
  })

  it('gives nothing for a class the corpus does not declare', () => {
    expect(schemaRows('sasquatch')).toEqual([])
  })
})
