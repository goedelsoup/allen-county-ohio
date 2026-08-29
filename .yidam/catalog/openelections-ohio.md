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
    value: https://api.github.com/repos/openelections/openelections-data-oh/contents/2020
    description: Directory listing for the 2020 files; other years are siblings
used-by:
  - ../corpus/division/voting-district-lima-1a-2020.yml
  - ../corpus/measure/allen-county-presidential-vote-2020.yml
  - ../corpus/measure/allen-county-turnout-2020.yml
  - ../corpus/measure/allen-county-voting-districts-2020.yml
  - ../corpus/place/lima.yml
  - ../corpus/question/who-lives-in-the-county-without-housing.yml
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

**What it cannot support.** One election. Nothing here is a series, a swing, or a precinct's
history — see [eighty-eight-rows-are-not-eighty-eight-nodes](../decisions/eighty-eight-rows-are-not-eighty-eight-nodes.yml),
which named returns as the retrieval that would make precincts differ in kind and gets a narrower
answer than it expected. It also carries no primaries, no local offices, no levies and no issues,
which for a county reference is most of what a board of elections actually runs.
