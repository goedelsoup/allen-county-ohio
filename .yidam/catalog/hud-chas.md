---
name: Comprehensive Housing Affordability Strategy data (HUD)
description: >-
  What housing costs the households who live in it, cross-tabulated the way federal housing money is
  allocated: by tenure, by income measured against the area median, by how much of that income the
  rent or mortgage takes, and by whether the household's own income band could afford the unit it is
  in. Custom tabulations of the American Community Survey, published for every county, place and
  tract in the country.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 365
location:
  - kind: url
    value: https://www.huduser.gov/portal/datasets/cp/2018thru2022-050-csv.zip
    description: >-
      The newest county file, 17.6 MB, 24 tables, 3,222 county rows. The path is a template —
      `<start>thru<end>-<summary level>-csv.zip` — and the vintages that answer are 2006thru2010,
      2009thru2013, 2011thru2015, 2012thru2016, 2015thru2019, 2016thru2020, 2017thru2021 and
      2018thru2022. `2019thru2023` and later return 404, so this is the current edition.
  - kind: url
    value: https://www.huduser.gov/portal/datasets/cp/2011thru2015-050-csv.zip
    description: >-
      The 2011–2015 window, used here as the earlier reading. 3,220 county rows and the same 24
      tables, **plus the data dictionary and a readme that the 2018–2022 zip does not contain**.
  - kind: url
    value: https://www.huduser.gov/portal/datasets/cp/2015thru2019-050-csv.zip
    description: >-
      The 2015–2019 window, which matters because it ends before the pandemic and so tests whether a
      change seen in the 2018–2022 file is an artefact of two years of emergency income.
  - kind: url
    value: https://www.huduser.gov/portal/datasets/cp/2006thru2010-050-csv.zip
    description: >-
      The oldest window. Its zip has **no `050/` folder** — the tables sit at the root — and it has
      23 tables rather than 24, Table 6 (disability) not yet existing.
  - kind: url
    value: https://www.huduser.gov/portal/datasets/cp/CHAS-data-dictionary-18-22.xlsx
    description: >-
      The dictionary for the current file, which must be fetched separately. `-17-21` and the other
      windows follow the same pattern. Without it the data are unreadable; with it, its `All Tables`
      sheet gives 4,945 rows of column definitions.
  - kind: url
    value: https://www.huduser.gov/portal/datasets/cp/2018thru2022-160-csv.zip
    description: >-
      The same tables by census place, 84 MB, 32,186 rows of which 1,265 are Ohio. This is where
      Lima separates from the county. Summary level 140 (tract) is published too; 150 is not.
used-by:
  - ../corpus/measure/allen-county-housing-cost-burden-2006-2022.yml
  - ../corpus/question/why-hud-and-the-survey-count-different-assisted-renters.yml
---

**The columns are named `T8_est69` and the dictionary is not in the box.** Every table is 152 to 300
columns of `T<n>_est<k>` and `T<n>_moe<k>`, and nothing in the 2018–2022 zip says what any of them
means. The dictionary is a separate workbook on the download page. [verified] — the retrievals
above. The 2011–2015 zip ships it inside; the current one does not, so a reader who worked from the
newest file alone would have 24 tables of unlabelled integers.

**Below 12, the only values that exist are 0, 4 and 10.** Across the 235,206 estimate cells of one
county table, every value is zero, four, or a multiple of five — there is no 1, 2, 3, 5, 6, 7, 8, 9
or 11 anywhere in the file. [verified] — `Table9.csv`, 2018–2022 county file. **The 4 is a
disclosure floor and not a count**, and subtotals containing one do not add: Allen County's
American Indian owner-occupied households read 4 while their four burden categories read 4, 0, 4
and 0. [verified] — the same table.

**Rounding shows up as a whole-county discrepancy.** Allen County's seven renter race rows sum to
13,245 against the file's own renter total of 13,240, and the seven owner rows to 27,489 against
27,495. [verified] — the same table. Neither is an error to be found and fixed; both are what
rounding 14 numbers to the nearest five does. A node that reports a share must say which
denominator it used.

**Every estimate has a margin and they are not small.** Allen County has 2,540 renter households at
or below 30 per cent of area median income, plus or minus 359. [verified] — `Table8.csv`, columns
`T8_est69` and `T8_moe69`. See [a survey is not a count](../decisions/a-survey-is-not-a-count.yml):
these are American Community Survey estimates wearing a different set of column names, and the
five-year window is the survey's, not a year.

**The windows overlap and are not a series.** 2015–2019 and 2018–2022 share two years of sample;
2016–2020, 2017–2021 and 2018–2022 share four. Four windows are used in this corpus — 2006–2010,
2011–2015, 2015–2019 and 2018–2022 — and only the first three pairs are close to independent. A
change measured between two adjacent vintages is measured mostly against itself.

**HAMFI is the unit of account and it is not the county's median income.** Every income band here is
a share of *HUD Area Median Family Income* for the Lima metropolitan area, a figure HUD computes and
publishes for its programmes, and the affordability bands `RHUD30`, `RHUD50` and `RHUD80` are the
rents a household at those shares could pay without spending more than 30 per cent of income.
Nothing in these files states the dollar value of either. [verified] — the dictionary's variable
list.

**What it is for, and the shape that follows.** These are the tables that decide who gets housing
money, so their cross-tabulations are the ones a programme needs: income band by cost burden by
tenure (Tables 7, 8, 9), and the unit's affordability against its occupant's income band (Tables
14A–15C, 17A–18C). That second family answers a question the American Community Survey alone cannot
— not how many cheap homes a county has, but how many of them a poor household could actually move
into. [verified] — the table list.

**The bot filter is the same one the rest of huduser.gov uses.** A request without a browser
user-agent gets a success and an empty body. [verified] — see
[A Picture of Subsidized Households](hud-picture-of-subsidized-households.md), which documents it at
length.
