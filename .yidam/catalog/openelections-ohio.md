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
  - kind: url
    value: https://api.github.com/repos/openelections/openelections-data-oh/git/trees/master?recursive=1
    description: >-
      The whole repository in one response — 75 files across twelve years. It is how the five
      general elections below were found; the per-directory listing above had been read once, for
      2020, and the years before 2012 were never looked at.
  - kind: url
    value: https://raw.githubusercontent.com/openelections/openelections-data-oh/master/2006/gen06statewide.csv
    description: >-
      **The Secretary of State's own 2006 workbook**, 2.8 MB, one row per precinct with 40 columns:
      registered voters, ballots cast, six statewide executive offices, US Senate, US House, both
      chambers of the General Assembly, two Ohio Supreme Court seats and four state ballot issues.
      Allen County is 139 rows. This is not a transcription — it is the office's own file, carried
      in the volunteers' repository.
  - kind: url
    value: https://raw.githubusercontent.com/openelections/openelections-data-oh/master/2010/2010_general_precinct.txt
    description: >-
      The Secretary of State's 2010 file, 7.8 MB, tab-separated, with `REG_VOTERS` and
      `TOTAL_VOTERS` and a column per candidate. **Its last Allen County row is `ZZZ / COUNTY
      TOTALS`**, so a sum over the file's rows returns exactly twice the county.
  - kind: url
    value: https://raw.githubusercontent.com/openelections/openelections-data-oh/master/2002/20021105__oh__general.csv
    description: >-
      2002 general, county grain — governor and the five other statewide executive offices. 2008
      and 2014 are the same shape at `2008/20081104__oh__general.csv` and
      `2014/20141104__oh__general.csv`; 2006 and 2010 exist only at precinct grain and are summed.
used-by:
  - ../corpus/measure/allen-county-presidential-vote-2000-2016.yml
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

**It reaches back to 2000, and this corpus had read three years of it.** The Ohio repository holds
general-election files for 2000, 2008, 2012, 2016 and 2020 — the presidential vote read here for
2016 and 2020 came from the last two of those, and 2000, 2008 and 2012 sat unread beside them.
[verified] — the repository's own listing, read 2026-09-02. It has **no general-election file for
2004**: that directory holds a March primary and a parser, so the one presidential election in the
run this project cannot supply is the one
[the other compilation](medsl-county-returns.md) had to.

**It carries a source it did not write, and that source outranks it.** `2012/FinalResults.xlsx` in
the same directory is the Ohio Secretary of State's own final-results workbook for the 2012
general — a `Contents` sheet, a `Master Sheet`, and one sheet per office, with a county to a row.
Its `President` sheet gives Allen County Obama 17,914, Romney 29,502 and a presidential total of
48,236. [verified] — the workbook, read here. That is the certified figure and the transcription
beside it agrees with it exactly, which is the strongest thing this corpus can currently say about
any modern county return: the compiler was checked against the compiled and did not drift.

**And it is still not the canvass.** The workbook is the Secretary of State's, and the Secretary of
State's is a compilation of eighty-eight county boards' certificates, none of which is held here.
[inference] The site that would serve them
[returns 403 to every client tried](allen-county-official-site.md).
**Five general elections were in this repository and unread.** 2002, 2006, 2008, 2010 and 2014 all
carry Allen County, and the entry above described the repository as holding 2012, 2016, 2018 and
2020 because the directory listing had been read for one year only. [verified] — the recursive tree
listing. The cost of that was a corpus with one gubernatorial election in it and an inference drawn
from three consecutive ballots; see
[a sample is not a series](../decisions/a-sample-is-not-a-series.yml), whose second case this is.

**Vote counts are written as floating-point numbers in the 2006 and 2010 precinct files.** Allen
County's rows give `133.0`, `256.0`, `18000.0`. Every value checked is integral, and a reader that
casts to an integer without going through a float gets nothing: the parse fails, the row is skipped,
and a county sums to zero rather than raising. It did here, twice, before the values were looked at.
[verified] — the two files. The `district` column is a float too: a 2006 governor row carries
district `4.0` for an office that has no district.

**The 2010 precinct file has no party column at all.** Six offices, candidates given as
`Kasich, John` and `Strickland, Ted`, and nothing that says which party either belongs to. Party for
that year is carried in this corpus from the candidates and not from the file. [verified]

**The 2014 county file ends in two rows that are not rows.** After Allen County's U.S. House
figures — Garrett 7,603 and Jordan 19,660 — come two further Allen rows whose `office` column holds
`7603` and `19660` and whose every other field is empty. They are the same two numbers escaping
into the wrong column. [verified] A reader that groups by office silently gains two offices with no
votes; a reader that sums votes is unaffected, because the votes column is where the values are
missing.

**The Secretary of State's own two files carry a control the transcriptions do not.**
`REG_VOTERS` and `TOTAL_VOTERS` per precinct, in 2006 and 2010, are what let turnout be computed for
a year other than 2020 — and in 2010 the file's own `COUNTY TOTALS` row is the check, since the 121
precinct rows above it sum to it exactly: 69,931 registered and 33,867 cast. [verified] — the 2010
file. See [the control is the figure that cannot move](../decisions/the-control-is-the-figure-that-cannot-move.yml).

**Allen County's precinct count is in these files and it falls.** 139 precincts in 2006, 121 in 2010,
88 in 2014 and 88 in both 2018 and 2020 — a 37 per cent consolidation over eight years. [verified] —
the precinct grain of each file, counted by distinct code.

**Four statewide ballot issues, which is a kind of return this corpus had not held.** The 2006
workbook carries `STATE_ISSUE_2` through `STATE_ISSUE_5`, yes and no, per precinct. **It numbers them
and does not name them**, and no other source here does either. [verified]
