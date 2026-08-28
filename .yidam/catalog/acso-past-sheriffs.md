---
name: Allen County Sheriff's Office — Past Sheriffs
description: >-
  The Allen County Sheriff's Office's own published roster of the county's sheriffs, from
  Henry Lippencott in 1831 to the present holder.
type: other
obtained: true
retrieved: 2026-08-28
ttl_days: 365
location:
  - kind: url
    value: https://www.acso-oh.us/historical-information/past-sheriffs/
    description: the roster page, listing each holder with a year range
used-by:
  - ../corpus/office/allen-county-sheriff.yml
  - ../corpus/person/aaron-fisher.yml
  - ../corpus/person/alexander-beatty.yml
  - ../corpus/person/benjamin-s-miller.yml
  - ../corpus/person/charles-h-williams.yml
  - ../corpus/person/charles-w-baxter.yml
  - ../corpus/person/charles-w-harrod.yml
  - ../corpus/person/clay-t-cotterman.yml
  - ../corpus/person/daniel-w-beck.yml
  - ../corpus/person/donald-f-sarber.yml
  - ../corpus/person/edward-l-fair.yml
  - ../corpus/person/elias-a-bogart.yml
  - ../corpus/person/eugene-barr.yml
  - ../corpus/person/f-m-watt.yml
  - ../corpus/person/harvey-b-crosson.yml
  - ../corpus/person/henry-lippencott.yml
  - ../corpus/person/henry-van-gunter.yml
  - ../corpus/person/hiram-stotts.yml
  - ../corpus/person/isaac-bailey.yml
  - ../corpus/person/james-a-colbath.yml
  - ../corpus/person/james-k-everett.yml
  - ../corpus/person/jess-l-sarber.yml
  - ../corpus/person/john-franks.yml
  - ../corpus/person/john-keller.yml
  - ../corpus/person/john-w-cook.yml
  - ../corpus/person/lawrence-oneill.yml
  - ../corpus/person/mathias-ridenour.yml
  - ../corpus/person/matthew-b-treglia.yml
  - ../corpus/person/mp-hoagland.yml
  - ../corpus/person/ralph-s-marshall.yml
  - ../corpus/person/samuel-a-crish.yml
  - ../corpus/person/samuel-buckmaster.yml
  - ../corpus/person/samuel-collins.yml
  - ../corpus/person/sherman-e-eley.yml
  - ../corpus/person/william-h-harter.yml
  - ../corpus/person/william-miller.yml
  - ../corpus/person/william-r-dailey.yml
  - ../corpus/person/william-tingle.yml
  - ../corpus/person/william-v-daley.yml
  - ../corpus/question/two-irregular-sheriff-transitions.yml
  - ../corpus/question/when-allen-county-was-organized.yml
  - ../corpus/tenure/sheriff-1831-henry-lippencott.yml
  - ../corpus/tenure/sheriff-1835-john-keller.yml
  - ../corpus/tenure/sheriff-1839-alexander-beatty.yml
  - ../corpus/tenure/sheriff-1843-john-keller.yml
  - ../corpus/tenure/sheriff-1845-charles-h-williams.yml
  - ../corpus/tenure/sheriff-1849-hiram-stotts.yml
  - ../corpus/tenure/sheriff-1853-mathias-ridenour.yml
  - ../corpus/tenure/sheriff-1855-william-tingle.yml
  - ../corpus/tenure/sheriff-1857-samuel-buckmaster.yml
  - ../corpus/tenure/sheriff-1861-samuel-collins.yml
  - ../corpus/tenure/sheriff-1865-isaac-bailey.yml
  - ../corpus/tenure/sheriff-1869-james-a-colbath.yml
  - ../corpus/tenure/sheriff-1873-william-miller.yml
  - ../corpus/tenure/sheriff-1877-john-franks.yml
  - ../corpus/tenure/sheriff-1881-william-h-harter.yml
  - ../corpus/tenure/sheriff-1885-mp-hoagland.yml
  - ../corpus/tenure/sheriff-1889-lawrence-oneill.yml
  - ../corpus/tenure/sheriff-1893-aaron-fisher.yml
  - ../corpus/tenure/sheriff-1898-elias-a-bogart.yml
  - ../corpus/tenure/sheriff-1902-eugene-barr.yml
  - ../corpus/tenure/sheriff-1906-henry-van-gunter.yml
  - ../corpus/tenure/sheriff-1910-f-m-watt.yml
  - ../corpus/tenure/sheriff-1914-sherman-e-eley.yml
  - ../corpus/tenure/sheriff-1918-charles-w-baxter.yml
  - ../corpus/tenure/sheriff-1923-harvey-b-crosson.yml
  - ../corpus/tenure/sheriff-1927-john-w-cook.yml
  - ../corpus/tenure/sheriff-1929-benjamin-s-miller.yml
  - ../corpus/tenure/sheriff-1931-jess-l-sarber.yml
  - ../corpus/tenure/sheriff-1933-donald-f-sarber.yml
  - ../corpus/tenure/sheriff-1935-ralph-s-marshall.yml
  - ../corpus/tenure/sheriff-1937-william-v-daley.yml
  - ../corpus/tenure/sheriff-1945-william-r-dailey.yml
  - ../corpus/tenure/sheriff-1953-clay-t-cotterman.yml
  - ../corpus/tenure/sheriff-1965-edward-l-fair.yml
  - ../corpus/tenure/sheriff-1977-charles-w-harrod.yml
  - ../corpus/tenure/sheriff-1992-daniel-w-beck.yml
  - ../corpus/tenure/sheriff-2009-samuel-a-crish.yml
  - ../corpus/tenure/sheriff-2017-james-k-everett.yml
  - ../corpus/tenure/sheriff-2017-matthew-b-treglia.yml
---

An office's own record of who has held it. That makes it a primary source for this particular
fact and not for much else: the office is the custodian of its own succession, and no
aggregator or county history is closer to it.

**What the roster gives.** Thirty-nine entries, each a year range and a name, ordered most
recent first, running continuously from 1831 to the current holder. It gives no month or day,
no manner of taking or leaving office, and no biographical detail whatever.

**What it therefore cannot support.** Every `how_began` and `how_ended` in this corpus's
tenure nodes is absent rather than guessed, because this source does not say. Two entries
visibly invite a guess and get none: `1931-1933 Jess L. Sarber` followed by
`1933-1935 Donald F. Sarber`, and a single-year `2017-2017 James K. Everett` between two
longer holders. Both patterns have obvious readings and the roster states neither. See
[the two irregular sheriff transitions](../corpus/question/two-irregular-sheriff-transitions.yml).

**A spelling to preserve.** The roster reads `Henry Lippencott`. Secondary sources render the
same man `Lippincott`. This corpus follows the roster and records the variant, because the
office's own spelling of its first sheriff is the better authority and because silently
normalizing it would destroy the only evidence that the two spellings refer to one person.

**A precision artifact worth knowing before running anything over these nodes.** Ranges are
year-only, so consecutive tenures share a boundary year — `2009-2017` and `2017-2017` and
`2017-current` all touch 2017. Read as intervals these overlap; read as the source intends
they do not. A succession audit that treats year precision as day precision will report
thirty-eight overlaps in a roster that has none.

**A date it corroborates.** Lippencott's term begins in 1831, which is independent support for
the county's government having been organized that year rather than at its 1820 erection —
see [when Allen County's government was organized](../corpus/question/when-allen-county-was-organized.yml).

**`ttl_days` is 365** because the current holder's entry changes with an election, and a
roster read once is stale the next time the office turns over.
