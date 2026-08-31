---
name: USDA Census of Agriculture, county files 2002–2022
description: >-
  The federal agricultural census, taken every five years, in the only form that is public without
  a key: NASS's Quick Stats bulk extracts. It is the first agricultural source this corpus has held
  since the 1910 volume, and it closes a 113-year gap in the county's largest land use.
type: dataset
obtained: true
retrieved: 2026-08-30
ttl_days: 1825
location:
  - kind: url
    value: https://www.nass.usda.gov/datasets/qs.census2022.txt.gz
    description: >-
      The 2022 census as one tab-separated file, 309 MB gzipped, every state, county, district and
      commodity. `qs.census2017`, `qs.census2012`, `qs.census2007` and `qs.census2002` are the same
      file for the earlier censuses; `qs.census1997` does not exist, and 2002 is where the series
      begins. Filter to `AGG_LEVEL_DESC == COUNTY`, `STATE_ALPHA == OH`, `COUNTY_NAME == ALLEN`,
      which leaves between 1,014 and 2,097 rows a year.
  - kind: url
    value: https://www.nass.usda.gov/Publications/AgCensus/2022/Online_Resources/County_Profiles/Ohio/cp39003.pdf
    description: >-
      The two-page county profile, which is the same census rendered for a reader. Useful as a
      second witness on the headline figures and as the only place the sales ranks are printed.
      The 2017 profile is at the same path with the year changed; the 2012 one is not published.
  - kind: url
    value: https://www.nass.usda.gov/Publications/AgCensus/2022/Full_Report/Volume_1,_Chapter_2_County_Level/Ohio/ohappxa.pdf
    description: >-
      Appendix A, the methodology. It carries the definition of a farm, the disclosure rules behind
      every `(D)`, and Table C, which prints a standard error and an adjustment share for every
      county in the state. It is the only place the census says how much of a county figure is a
      returned form.
used-by:
  - ../corpus/measure/allen-county-crops-2022.yml
  - ../corpus/measure/allen-county-farms-2002-2022.yml
  - ../corpus/measure/allen-county-livestock-2002-2022.yml
  - ../corpus/measure/allen-county-domestic-animals-1910.yml
  - ../corpus/measure/allen-county-farms-1900-1910.yml
  - ../corpus/place/allen-county.yml
  - ../corpus/question/when-the-farmland-went.yml
---

**It closes a gap the corpus had named twice.** Both
[the county node](../corpus/place/allen-county.yml) and
[the 1910 animals](../corpus/measure/allen-county-domestic-animals-1910.yml) carried an `[open]`
saying that the Thirteenth Census was the only agricultural census this corpus had read, and that
nothing in it showed whether the farmland had gone or when the horses went. Five censuses answer
the first and not the second. [verified]

**Its finding is not on the page the census wrote for a reader.** The county profile leads with
farms, land in farms and market value, all three of which barely move. The thing that moved is an
inventory row on the second page: **Allen County held 31,741 hogs on 59 operations in 2002 and
235,800 on 28 in 2022**, and the twelve operations raising them under production contract turned
out 281,194 head — three quarters of everything the county sold. [verified] See
[the livestock](../corpus/measure/allen-county-livestock-2002-2022.yml).

**The bulk files are the whole census and the profile is an excerpt.** Between 1,014 and 2,097 rows
a year survive the county filter, against the sixty-odd figures the profile prints, and the extra
rows are where the tenure split, the contract production, the expense detail and the per-operation
counts live. Four of the five findings this corpus took from the source are in rows the profile does
not carry.

**Appendix A prices the headline.** Table C gives Allen County 897 farms with a standard error of
69, and 178,921 acres with a standard error of 16,171. The profile prints those same figures as up
5 per cent and down 4 per cent on 2017; the first change is smaller than one standard error and the
second smaller than half of one. [verified] The same table says **39.0 per cent of the county's farm
count and 28.6 per cent of its acreage are adjustment** — for coverage, nonresponse and
misclassification — rather than a returned form. A census that publishes this about itself in an
appendix is more honest than most sources here, and a reader of the profile alone would never learn
it.

**Its sales row is not read.** Table C gives Allen County's market value as 240,849 with a standard
error of 15, in a table whose other rows carry standard errors of 69 on 897 and 16,171 on 178,921.
Fifteen thousand dollars on two hundred and forty million is not a plausible standard error and the
column heading names no unit for this block, so the figure is left alone rather than reconciled by
guess. [open]

**`(D)` is a rule, not a small number**, and the rule is one this corpus already follows. Appendix A:

> The threshold rule failed if the data cell contained less than three operations. For example, if
> only one farmer produced turkeys in a county, NASS could not publish the county total for turkey
> inventory without disclosing that individual's information. The dominance rule failed if the
> distribution of the data within the cell allowed a data user to estimate any respondent's data
> too closely.

Allen County's milk-cow inventory was 228 head on five operations in 2012 and is withheld in 2022
on eight, which is the dominance rule rather than the threshold one. Its rye acreage is withheld
because a single operation in the county grows rye. See
[what may leave the repository](../decisions/what-may-leave-the-repository.yml), which reaches the
same rule from the other end.

**Its definition of a farm is not the 1910 census's.** Appendix A: "any place from which $1,000 or
more of agricultural products were produced and sold, or normally would have been sold, during the
census year." The Thirteenth Census counted land farmed by one manager, excluding anything under
three acres that produced less than $250. Two counts on two thresholds are not a series, which is
why this corpus compares acres, animals and bushels across the century and does not subtract 897
from 2,939. See [a farm count is a definition](../decisions/a-farm-count-is-a-definition.yml).

**One discrepancy between its own two publications, unresolved.** The 2022 county profile's race
table gives Allen County no American Indian or Alaska Native producers — its six race rows sum to
the county's 1,564 producers exactly — while Appendix A Table D gives seven. The obvious
reconciliation is that one counts a race reported alone and the other alone or in combination, and
neither page says so. [open]

**What is in these files and unread.** Producer characteristics by age, sex and years on the
operation; farms by North American Industry Classification System code; fertilizer, chemical and
irrigation practice; energy expense; and the whole of the 2007 and 2012 censuses beyond the four
rows this phase took from them. The 1997 census and everything before it is not here at all —
`qs.census1997.txt.gz` returns 404 and 2002 is where the series begins. See
[when the farmland went](../corpus/question/when-the-farmland-went.yml).
