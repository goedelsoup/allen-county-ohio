---
name: Election Administration and Voting Survey (U.S. Election Assistance Commission)
description: >-
  Every county board of elections in the country answers the same questionnaire after every federal
  election: how many people were on the roll, how many were taken off and why, how many ballots were
  cast and by which method, how many polling places and poll workers there were, and how many
  provisional and mail ballots were rejected. It runs from 2004 to 2024 and it is the only source
  this corpus holds that describes the machinery of an election rather than its result.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 365
location:
  - kind: url
    value: https://www.eac.gov/research-and-data/eavs-retrospective
    description: >-
      The harmonised panel, 2004 to 2022, published as one workbook per survey section — A
      registration, B overseas and military, C mail ballots, D polling places and poll workers, E
      provisional ballots, F participation. 61,261 jurisdiction-years in Section A alone. Allen
      County is `State_Abbr` OH with `FIPSCode` 3900300000.
  - kind: url
    value: https://www.eac.gov/sites/default/files/2026-02/2024_EAVS_for_Public_Release_nolabel_V2_csv.zip
    description: >-
      The 2024 survey, version 2 of February 2026, 535 columns and 6,377 jurisdictions in one file.
      The retrospective stops at 2022, so the most recent presidential election has to be fetched
      separately and joined by hand.
  - kind: url
    value: https://www.eac.gov/sites/default/files/2023-06/2022_EAVS_Codebook.xlsx
    description: >-
      What each variable name means. `F1a` is total ballots counted, `A1a` the roll, `A9e` removals
      for failure to respond to a confirmation notice, `D7a` poll workers, `E1d` provisional
      rejections. Nothing in the data files carries a label.
used-by:
  - ../corpus/measure/allen-county-turnout-2004-2024.yml
  - ../corpus/measure/allen-county-voter-roll-removals-2006-2022.yml
  - ../corpus/measure/allen-county-polling-places-2004-2024.yml
---

**Negative numbers are not counts.** `-66` means the question was not asked that year, `-77` that
the jurisdiction did not answer, `-88` that the question does not apply to it, and `-99` that the
answer is unavailable. Summed naively they subtract; treated as zero they invent zeros. [verified] —
the codebook. Allen County's roll of registered voters reads `-99` for 2010, 2020 and 2022 in the
*inactive* column and the same figure as the total in the *active* column, which is a reporting
convention and not a county without inactive voters.

**The jurisdiction's name changes and its key does not.** This county appears as `ALLEN COUNTY` in
2004, 2006, 2020, 2022 and 2024 and as `ALLEN` in 2008, 2010, 2012, 2014, 2016 and 2018; its
`FIPSCode` is `39003` in 2008 and `3900300000` in every other year. [verified] — the Section A
panel. An exact match on `ALLEN COUNTY` loses six of the eleven elections and a match on the ten-digit code loses 2008.
[HUD's subsidised-housing file](hud-picture-of-subsidized-households.md) writes this county's name
two different ways for the same reason, and the fix is the same: join on the code.

**The parts sum to the whole, except where they do not, and that is the check.** Ballots by method —
election day, early in person, mail, overseas, provisional — sum to the reported total exactly in
ten of the eleven Allen County years. In 2024 they fall 37 short of 46,845. [verified] — the same
files. Poll workers by age band sum to the reported total in every year from 2012 on; in 2010 they
sum to 895 against a reported 531, and in 2006 the reported total is 139, which is that year's
precinct count and not a number of people.

**Three figures in it are already in this corpus from another pipeline, and they agree exactly.**
Ballots cast in Allen County were 37,605 in 2006, 33,867 in 2010 and 48,353 in 2020 on the certified
precinct returns, and the survey gives the same three numbers. [verified] — this file against
[OpenElections](openelections-ohio.md). The registered-voter counts agree for 2010 and 2020 and
differ by 2,337 for 2006.

**It describes the administration and not the vote.** There are no candidates in it and no party
identification. A question about who won is a question for
[OpenElections](openelections-ohio.md) or the printed
[state election statistics](ohio-election-statistics-1888-1910.md); this file answers how the
ballots got there.
