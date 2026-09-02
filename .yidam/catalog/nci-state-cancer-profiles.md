---
name: State Cancer Profiles (NCI and CDC)
description: >-
  County-level cancer death and incidence rates by site, sex and race, with the state and national
  figures beside them, the county's rank among Ohio's 88, and — unusually — a confidence interval
  on that rank. It is the door that opened where CDC WONDER's is shut.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 365
location:
  - kind: url
    value: https://statecancerprofiles.cancer.gov/deathrates/index.php?stateFIPS=39&areatype=county&cancer=001&race=00&sex=0&age=001&year=0&type=death&sortVariableName=rate&sortOrder=default&output=1
    description: >-
      Death rates for all cancer sites, every Ohio county, 2019–2023. `output=1` returns CSV rather
      than a page. `cancer=` selects the site, `race=` the group — `00` all, `07` White
      non-Hispanic, `28` Black non-Hispanic — and `sex=` is `0`, `1` or `2`. The columns read here
      are the age-adjusted rate and its interval, `CI*Rank` and its interval, the average annual
      count and the recent trend.
  - kind: url
    value: https://statecancerprofiles.cancer.gov/incidencerates/index.php?stateFIPS=39&areatype=county&cancer=001&race=00&sex=0&age=001&stage=999&year=0&type=incd&sortVariableName=rate&sortOrder=default&output=1
    description: >-
      Incidence for the same geography, 2018–2022, on a different window from mortality and with an
      extra `stage=` parameter. Its table carries a state row and **no national row**, so a county
      can be set against Ohio here and against the United States only on the mortality side.
used-by:
  - ../corpus/measure/allen-county-cancer-2018-2023.yml
---

**The site codes are not guessable and not documented in the response.** `001` is all sites, `047`
lung, `040` pancreas, `035` liver, `020` colon and rectum, `055` breast, `066` prostate, `017`
oesophagus, `053` melanoma, `061` ovary, `071` bladder, `072` kidney, `076` brain, `086`
non-Hodgkin lymphoma, `090` leukemia, `018` stomach, `057` cervix, `058` uterus, `080` thyroid. A
code with no dataset behind it returns an HTML page with HTTP 200, so a caller that does not check
the first line of the body will parse a web page as a table and find nothing. [verified] — the
endpoint, twenty-eight codes tried.

**It publishes an interval on the rank, and the interval is wide.** Allen County's all-sites death
rank is 62nd of 88 with a 95 per cent interval of 29th to 80th; its pancreas rank is 16th with an
interval of 2nd to 73rd. A county rank computed from point estimates alone — which is what this
corpus has done elsewhere — is a point estimate of a rank and carries uncertainty of this order.
[verified] — same source, the `CI*Rank` columns. See
[a rank is an estimate](../decisions/a-rank-is-an-estimate.yml).

**A site with three or fewer deaths a year is suppressed, and four of this county's nineteen are.**
Stomach, cervix, uterus and thyroid return `*` for the rate and `3 or fewer` for the count. That is
a statement about the county's size, not about the disease: Ohio publishes all four. [verified] —
same source.

**Mortality and incidence do not cover the same years.** Deaths are 2019–2023 and cases 2018–2022,
so a ratio of the two is not a survival rate and is not computed here. [verified] — the two report
headers.

**Race is offered as non-Hispanic categories only for a county this size.** `race=01` and `race=02`
return a web page; `07` and `28` return data. [verified] — the endpoint, four codes tried.
