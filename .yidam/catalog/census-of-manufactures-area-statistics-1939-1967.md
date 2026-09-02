---
name: Census of Manufactures, Ohio area statistics, 1939–1967
description: >-
  The five volumes that carry Allen County and Lima across the forty years between the 1929 state
  chapter this corpus already reads and the federal employment series that begins in 1969. Each
  prints the census before it as well as its own, so five volumes give six censuses. They are the
  answer to a negative finding this corpus recorded a phase earlier: the Census Bureau's own host
  holds no manufactures volume between 1929 and 1963, and archive.org holds all five.
type: publication
obtained: true
retrieved: 2026-09-02
ttl_days: 3650
location:
  - kind: url
    value: https://archive.org/details/sixteenthcensuso03unit
    description: >-
      Sixteenth Census of the United States: 1940 — Census of Manufactures, 1939, Volume III,
      Reports by States. 1,214 leaves, 99 MB. Ohio's chapter opens at book page 765. Table 2,
      *Summary for industrial areas, counties, and places: 1939*, gives Allen County on leaf 781
      and Lima on leaf 783. Book page = leaf − 15.
  - kind: url
    value: https://archive.org/details/1954censusofmanu03unse
    description: >-
      1954 Census of Manufactures, Volume III — Area Statistics. 1,080 leaves, 86 MB. Ohio is
      chapter 134. Table 3, *General statistics for standard metropolitan areas, counties, and
      urban places: 1954 and 1947*, has the Lima area and Allen County on book page 134-5 (leaf
      666) and Lima city on 134-7 (leaf 668). The area definitions on leaf 70 state that the Lima
      standard metropolitan area is coextensive with Allen County.
  - kind: url
    value: https://archive.org/details/1958censusofmanu03unse
    description: >-
      1958 Census of Manufactures, Volume III — Area Statistics. 1,118 leaves, 98 MB. Ohio is
      chapter 34 and book page 34-N is leaf N + 695. Table 3, *General statistics for standard
      metropolitan statistical areas, counties, and selected cities: 1958 and 1954*, begins at
      34-7 (leaf 702) with Allen County on that page and Lima city on 34-9 (leaf 704). Table 6,
      *General statistics for selected counties by industry group: 1958 and 1954*, begins at 34-24
      (leaf 719) and Allen County is its first entry.
  - kind: url
    value: https://archive.org/details/1963censusofmanu03unse
    description: >-
      1963 Census of Manufactures, Volume III — Area Statistics. 1,334 leaves, 104 MB. Ohio is
      chapter 36 and book page 36-N is leaf N + 897. Table 4, *General statistics for standard
      metropolitan statistical areas, counties, and selected cities: 1963 and 1958*, has Allen
      County on 36-9 (leaf 906) and Lima city on 36-11 (leaf 908).
  - kind: url
    value: https://archive.org/details/1967censusofmanu32unse
    description: >-
      1967 Census of Manufactures, Volume III — Area Statistics, Part 2, Nebraska–Wyoming. 692
      leaves, 48 MB. Ohio is chapter 36 and book page 36-N is leaf N + 205. Table 4, *General
      statistics for standard metropolitan statistical areas, counties, and selected cities: 1967
      and 1963*, has Allen County on 36-6 (leaf 211) and Lima city on 36-8 (leaf 213). Table 6,
      *General statistics for selected counties by industry group: 1967 and 1963*, has Allen County
      first, on leaf 234. This is the only one of the five that prints employment in thousands to
      one decimal rather than as a count.
used-by:
  - ../corpus/measure/allen-county-manufactures-1939-1967.yml
  - ../corpus/measure/lima-manufactures-1939-1967.yml
  - ../corpus/measure/allen-county-manufacturing-outside-lima-1929-1967.yml
---

**Every figure taken from these five volumes was read off a page image.** All five have an OCR
text layer and none of it is usable: the tables are set in twelve numeric columns and the OCR
returns them column by column, so no row can be reassembled from the text. Pages were located by
the item's own full-text search — which reports a leaf index one greater than the one archive.org's
page-image endpoint takes — and then rendered at the item's native resolution. [verified] — the
five items, 2 September 2026.

**Four population columns are the first control, and all four pass.** The 1939 volume prints a
1940 population beside each area: 73,303 for Allen County and 44,711 for Lima. The 1954 volume
prints a 1950 population: 88,183 and 50,246. Those are the four census counts this corpus already
holds from the state population volumes, and no manufacturing figure was written down until they
matched. [verified] — see
[the county by race, 1930–1960](../corpus/measure/allen-county-population-by-race-1930-1960.yml).

**The 1967 volume checks itself against two neighbouring counties.** By 1967 the Lima standard
metropolitan statistical area had grown from Allen County alone to Allen, Putnam and Van Wert, and
the volume prints all four rows. The three counties sum to the area in eleven of the twelve columns
exactly — establishments, establishments with twenty or more employees, employees, payroll,
production workers, wages, value added, cost of materials, value of shipments, capital expenditures
and the 1963 employment column. The twelfth, man-hours, sums to 35.5 against a printed 35.6, which
is what three figures rounded to a tenth do. [verified] — Table 4, the arithmetic performed here.

**Each volume reproduces the one before it, and that is the second control.** The 1958 volume's
1954 column gives Allen County 14,691 employees and $99,158 thousand of value added and Lima
11,073 and $69,511 — the same four figures the 1954 volume prints for itself. The 1967 volume's
1963 column gives 14.9 thousand employees and $213.8 million of value added against the 1963
volume's own 14,897 and $213,827 thousand. [verified] — the four volumes read against each other.

**Where they do not reproduce each other, the disagreement is small and it is recorded rather than
resolved.** Allen County's 1958 employment is printed three times: 13,526 in the 1958 volume, 13,534
in the 1963 volume's retrospective column, and 13,543 in the 1963 volume's Lima area row — which is
the same area, because the area was still coextensive with the county and the other ten columns of
those two rows are identical. Two of the three cannot both be revisions. The corpus takes the 1958
volume's own 13,526, notes the 1963 revision, and treats 13,543 as a transposition. The spread is
seventeen people on thirteen thousand. [verified] — the two volumes; see
[two volumes are not one series](../decisions/two-volumes-are-not-one-series.yml).

**The 1939 volume closes its own identity.** Allen County's value of products, $36,402,403, less
its cost of materials, supplies, fuel, purchased electric energy and contract work, $19,946,754,
is $16,455,649, which is the value added the table prints. [verified] — Table 2.

**What these volumes count changes once inside the run, and the break is between the first and the
second.** The 1939 volume counts *wage earners, average for the year*, which is the same concept
the 1919 and 1929 volumes use. From 1947 the tables count *all employees* and report production
workers as a subcolumn. The two are not the same number and this corpus does not difference them
without saying so. [verified] — the column headings themselves.

**What is not here.** No census of manufactures was taken for any year between 1939 and 1947, so
these volumes say nothing about what the county's factories did during the war they were built up
for. The 1947 volume itself was not needed — the 1954 volume carries 1947 — and was not sought.
