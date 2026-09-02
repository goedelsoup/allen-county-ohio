---
name: OpenElections — Ohio precinct-level returns
description: >-
  A volunteer project's machine-readable transcription of official certified election returns,
  precinct by precinct. The first source in this corpus that says how the county votes rather
  than where its lines run or who governs it.
type: dataset
obtained: true
retrieved: 2026-08-29
ttl_days: 1825
location:
  - kind: url
    value: https://raw.githubusercontent.com/openelections/openelections-data-oh/master/2020/20201103__oh__general__precinct.csv
    description: Ohio, 2020 general election, precinct grain — 23.7 MB, 1,408 rows for Allen County
  - kind: url
    value: https://raw.githubusercontent.com/openelections/openelections-data-oh/master/2016/20161108__oh__general__precinct.csv
    description: Ohio, 2016 general — 2,904 rows for Allen County
  - kind: url
    value: https://raw.githubusercontent.com/openelections/openelections-data-oh/master/2018/20181106__oh__general__precinct.csv
    description: Ohio, 2018 general — 2,200 rows for Allen County
  - kind: url
    value: https://api.github.com/repos/openelections/openelections-data-oh/contents/2020
    description: Directory listing; 2012, 2016 and 2018 are siblings, 2022 and 2024 have no precinct file
used-by:
  - ../corpus/division/voting-district-lima-1a-2020.yml
  - ../corpus/measure/allen-county-elected-seats-2026.yml
  - ../corpus/measure/allen-county-governor-vote-2018.yml
  - ../corpus/measure/allen-county-presidential-vote-2016.yml
  - ../corpus/measure/allen-county-presidential-vote-2020.yml
  - ../corpus/measure/allen-county-turnout-2020.yml
  - ../corpus/measure/allen-county-voting-districts-2020.yml
  - ../corpus/place/lima.yml
  - ../corpus/question/who-lives-in-the-county-without-housing.yml
  - ../corpus/question/why-allen-countys-villages-are-staffed-by-appointment.yml
---

**What it carries for this county.** 88 precincts, each with registered voters, ballots cast, and
votes by candidate for four offices: President, Representative to Congress, State Senator and
State Representative. 1,408 rows.

**It is a transcription, not the authority.** The authority for Ohio returns is the county board of
elections, certified through the Secretary of State, and the Secretary of State
[blocks automated clients](allen-county-official-site.md) — probed again on 2026-08-29 and still
403. OpenElections is volunteers converting the official PDFs and spreadsheets into CSV. That
makes it a **second-hand** source in a corpus whose others are publishers of their own data, and
every figure taken from it is a figure about what the transcription says.

**Two checks were run before anything was written from it.**

The first is a join. Its 88 precinct names are *identical* to the 88 `NAME` values
[TIGERweb](tigerweb-census2020.md) returns for Allen County's voting districts — same count, same
strings, no exceptions. Those are two entirely separate pipelines: a county board of elections
naming its own precincts, and the Census Bureau tabulating voting districts collected through the
Redistricting Data Program. That they agree string for string is the strongest evidence available
here that the transcription preserved the precinct identities.

The second is internal. Ballots cast across the 88 precincts total 48,353; presidential votes total
47,993. The difference of 360 is an undervote of 0.7 per cent, which is the ordinary size for a
presidential race and would not be if rows had been dropped or doubled.

**Three general elections, and the precinct names never move.** 2016, 2018 and 2020 were all
retrieved. All three give Allen County exactly 88 precincts, and the three name sets are
**identical** — no consolidation, no renaming, no split, no precinct appearing in one file and not
another across five years that included a decennial census and a statewide redistricting. Together
with the Census match already recorded above, four independently produced lists of this county's
precincts agree string for string.

**The schema changes at 2020 and the loss is turnout.** The 2016 and 2018 files carry
`county, precinct name, precinct code, office, district, party, candidate, votes` and **no**
`registered_voters` or `ballots_cast`. So this corpus can compare vote shares across the three
elections and can compare *turnout* only within 2020. Votes cast for the top office are the nearest
substitute and are not the same quantity: they exclude undervotes and every voter who came for
something else.

**What it cannot support.** No primaries, no local offices, no levies and no issues — which for a
county reference is most of what a board of elections actually runs. Nothing before 2012, and no
2022 or 2024 precinct file exists in the repository, so the series stops before Ohio's post-2020
maps took effect.

**It is no longer the only election source here, and the other one is the authority.** The Ohio
Secretary of State printed his own certified abstract of the 1920 returns, county by county, and
archive.org holds it: see
[Ohio Election Statistics, 1920](ohio-election-statistics-1920.md). That does not change what this
file is, and it does change what the corpus can say about it — the office whose present-day site
returns 403 to an automated client published the same class of figure in a book, and the book is
readable. The two do not overlap in time, so no figure here has yet been checked against one there.
