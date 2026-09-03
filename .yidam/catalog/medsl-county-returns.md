---
name: County Presidential Election Returns (MIT Election Data and Science Lab)
description: >-
  A single table of every county's presidential vote, compiled from the states' own canvasses by
  a university lab. It is the second compilation of these returns this corpus holds, and it exists
  here to be disagreed with: where it and OpenElections differ, the difference is the finding.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 365
location:
  - kind: url
    value: https://raw.githubusercontent.com/MEDSL/county-returns/master/countypres_2000-2016.csv
    description: >-
      4.5 MB, the lab's own public mirror of the dataset, covering 2000 through 2016. Allen County
      is `FIPS` 39003 and has sixteen rows — the two major candidates every year, plus a `green`
      row in 2000 and an `Other` row in each. This is the file that was read.
  - kind: url
    value: https://dataverse.harvard.edu/dataset.xhtml?persistentId=doi:10.7910/DVN/VOQCHQ
    description: >-
      The canonical deposit, now *County Presidential Election Returns 2000–2024*, version 20 of
      25 February 2026. It carries four more years than the mirror, including 2024. **It was not
      read.** Its data file is behind a guestbook — `You may not download this file without the
      required Guestbook response for guestbookID 458` — which is a form for a person to fill in,
      and the API request that skips it is not a way this corpus takes a source. So 2020 here comes
      from OpenElections and 2024 comes from nowhere.
used-by:
  - ../corpus/measure/allen-county-presidential-vote-2000-2016.yml
---

**What it is, and what it is not.** The lab compiles the states' certified county canvasses into
one schema. That makes it a **compilation**, in exactly the sense
[OpenElections](openelections-ohio.md) is one: neither publishes its own data, and both stand
between this corpus and the eighty-eight county boards whose certificates are the actual return.
Two compilations of one canvass are worth much more than one, and much less than the canvass.
[verified] — the dataset's own description.

**It disagrees with the other compilation, twice in five elections, by one vote.** For 2008 it
gives McCain 29,941 and Obama 19,521 where OpenElections gives 29,940 and 19,522 — one vote each
way, so the totals agree and no arithmetic anywhere catches it. For 2012 it gives Obama 17,913
where OpenElections gives 17,914, and here the totals differ too, 48,235 against 48,236.
[verified] — the two files, compared here.

**On 2012 it is wrong, and the arbiter is in the other repository.** OpenElections carries the Ohio
Secretary of State's own final-results workbook beside its transcription, and that workbook's
`President` sheet gives Allen County Obama 17,914, Romney 29,502 and a presidential total of
48,236. [verified] —
[the Ohio Secretary of State's 2012 workbook](openelections-ohio.md). The 2008 divergence has no
such arbiter held here and is published unresolved.

**It reaches one election this corpus could not otherwise have.** OpenElections has no Ohio general
file for 2004 — its 2004 directory holds a primary and a parser and nothing else — so 2004 is a
single-witness year, and this is the witness. [verified] — the repository's own listing.

**What it does not carry.** Nothing below the county: no precinct, no township, no ward. Every
sub-county figure in this corpus for 2016 and 2020 comes from
[OpenElections](openelections-ohio.md), and for 2000 through 2012 there is none.
