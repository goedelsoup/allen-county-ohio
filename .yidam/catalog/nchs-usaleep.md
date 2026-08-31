---
name: NCHS U.S. Small-area Life Expectancy Estimates Project (USALEEP)
description: >-
  Life expectancy at birth for almost every census tract in the United States, estimated by the
  National Center for Health Statistics from death certificates and population for 2010–2015. It is
  the only source in this corpus that says how long anyone here lives.
type: dataset
obtained: true
retrieved: 2026-08-31
ttl_days: 3650
location:
  - kind: url
    value: https://ftp.cdc.gov/pub/Health_Statistics/NCHS/Datasets/NVSS/USALEEP/CSV/OH_A.CSV
    description: >-
      Ohio, 2,773 tracts, one row each: tract identifier, life expectancy at birth, its standard
      error, and an abridged-life-table flag. Allen County has 30 rows. The same file under
      `www.cdc.gov/nchs/data/nvss/usaleep/` returns 403; the FTP mirror serves it.
used-by:
  - ../corpus/measure/allen-county-life-expectancy-2010-2015.yml
---

**Its geography is 2010 and the corpus's is 2020**, which is the first thing to know about it. Of
its 30 Allen County tracts, **28 carry an identifier that still exists in the 2020 geography**; two
— `39003010800` and `39003011300` — were split in the redraw and have no single successor. Seven
2020 tracts, holding 19,722 people, have no 2010 row at all. So this file covers 82,484 of the
county's 102,206 people and no arithmetic over it is a county figure. [verified]

**It publishes a standard error on every estimate**, between 0.97 and 2.48 years in this county,
which is what makes the spread here a finding rather than noise: 82.8 against 69.1 is more than
five combined standard errors apart. [verified]

**It is modelled, and not in the same way as [PLACES](cdc-places.md).** USALEEP builds an abridged
life table for each tract from actual death certificates and actual population, smoothed where the
counts are small; PLACES fits survey responses to demographic covariates. Both are estimates and
only one of them counts deaths. The distinction is why this corpus states a life expectancy and
declines to state a prevalence. See
[a modelled estimate is not an observation](../decisions/a-modelled-estimate-is-not-an-observation.yml).

**What is in it and unread.** The abridged-life-table flag, which marks how many age groups a
tract's table was built on — 1, 2 or 3 in this county — and is a quality signal this corpus has
recorded and not used. The companion files for other states, and the national file, which would put
Allen County's range against the country's rather than against Ohio's.
