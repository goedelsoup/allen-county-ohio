---
name: A Picture of Subsidized Households (HUD)
description: >-
  How many federally subsidized homes a county holds, how many people are in them, what those
  households earn, how old they are and what they pay — published once a year for every county,
  place, tract, housing authority and project in the country, and available here in an unbroken
  county series from 2004 to 2025.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 365
location:
  - kind: url
    value: https://www.huduser.gov/portal/datasets/pictures/files/COUNTY_2025_2020census.xlsx
    description: >-
      The county file, 2025 edition, 9.7 MB, one row per county per programme. The same path
      without the `_2020census` suffix serves 2009 through 2022; the suffix is required for 2023,
      2024 and 2025 and a request without it returns a 404 page named `.xlsx`.
  - kind: url
    value: https://www.huduser.gov/picture2008/2008_county.xlsx
    description: >-
      The 2004 to 2008 files, on an older path that has no `/portal/datasets/` in it. Five years
      that would be invisible to a reader who worked only from the current download page's newer
      links.
  - kind: url
    value: https://www.huduser.gov/portal/datasets/pictures/files/PLACE_2025_2020census.xlsx
    description: >-
      The same table by census place, 37 MB. This is where Lima separates from the county. Also
      published: tract, housing authority, project, CBSA, congressional district, ZIP, state and
      national files, all on the same naming scheme.
  - kind: url
    value: https://www.huduser.gov/portal/datasets/assthsg.html
    description: >-
      The download page, which lists every year's files for every geography and is the only place
      the 2004–2008 paths appear.
used-by:
  - ../corpus/measure/allen-county-subsidized-housing-2005-2025.yml
---

**The site answers a scripted request with a success and an empty body.** `curl` without a browser
user-agent gets HTTP 202 and zero bytes from every file above — no error, no redirect, no challenge
page. With a browser user-agent the same URLs return the workbooks. [verified] — every retrieval in
this entry. A retrieval script that checks the status code and not the size would record twenty-two
successful downloads of nothing.

**Every row is stamped with the quarter it describes.** The 2025 file carries `31DEC2025` in its
first column, so this source needs none of the guesswork that
[a file has more than one date](../decisions/a-file-has-more-than-one-date.yml) was written for. It
is also the reason the county series here can be dated year by year while HUD's
[live map layers](hud-open-data-housing-layers.md) cannot be dated at all.

**The header widens five times across the series and one column silently changes unit.** It runs 65
columns in 2004, 68 in 2009, 73 in 2014, 74 in 2015 and 88 in 2025; `name` becomes `NAME`; and
`total_occupied`, the count of subsidized units actually lived in, appears for the first time in the
2025 file. [verified] The unit change is the dangerous one: `hh_income` is in **thousands of
dollars** through 2008 and in **dollars** from 2009, under the same column name and with no note.
Allen County's assisted households read 10.1 in 2008 and 9,725.8 in 2009. [verified] — the 2008 and
2009 county files.

**Three negative sentinels, and they mean different things.** `-1` is unavailable, `-4` is
suppressed because the cell is too small to publish, `-5` is not applicable — a programme the
county does not have. Allen County's Mod Rehab row is `0` units and `-5` people every year from
2014, which is a programme that ended and not a programme with nobody in it. [verified] See
[a zero is not a blank](../decisions/a-zero-is-not-a-blank.yml).

**The programme roll changes in 2014 and the summary row survives it intact.** Through 2013 the
project-based side of the file is three categories — Section 8 NC/SR, Section 236, Multi-Family
Other. From 2014 it is one, Project Based Section 8, plus two new rows for 202/PRAC and 811/PRAC.
In Allen County the three 2013 categories hold 770, 96 and 105 units and the single 2014 category
holds 971, which is their sum to the unit. [verified] — the 2013 and 2014 county files. The
aggregate is comparable across the join and the parts are not, which is
[a revision that moves a category](../decisions/a-revision-that-moves-a-category.yml) in its
cleanest form.

**LIHTC is in the file for six years and never in the total.** Low-Income Housing Tax Credit units
are reported from 2008 to 2013 — 805 of them in Allen County in 2009, 865 in 2013 — and disappear
afterwards. They are excluded from `Summary of All HUD Programs` in every year they appear: the
2009 summary of 2,401 is 248 public housing plus 962 vouchers plus 840 plus 246 plus 105, with the
805 LIHTC units nowhere in it. [verified] — the 2009 county file. A reader who summed the programme
rows to check the total would find an 805-unit discrepancy and a reader who differenced 2013 against
2014 would find an 865-unit collapse, and neither is a change in the world.

**The control nearly closes, and the two units that are missing are Ohio's.** The 3,144 county rows
sum to 5,132,916 units against the national file's 5,132,918; Ohio's counties sum to 226,710 against
the state file's 226,712. [verified] — the county, state and national 2025 files. The gap is two
units in the whole country and it is entirely in one state, which is a rounding or assignment
artefact and not a suppression: no Ohio county row is blank or negative.

**Ohio has an eighty-ninth county in this file.** `39XXX Missing, Ohio` holds 640 subsidized units
that HUD could not assign to a county. [verified] Taking the county rows as the state is therefore
an undercount of a known and stated size, which is the honest form of the trap
[a county column is a filing decision](../decisions/a-county-column-is-a-filing-decision.yml)
describes.

**At place level a straddling city is one row.** Delphos city appears once with 26 voucher units and
Delphos lies in both Allen and Van Wert counties, so its units cannot be added to Allen's place
totals. [verified] — the 2025 place file; the straddle is the one this corpus already knows from
[a county column is a filing decision](../decisions/a-county-column-is-a-filing-decision.yml).
Lima, Bluffton, Spencerville, Elida and Fort Shawnee CDP are wholly within the county and can be.

**What it does not carry.** No waiting-list length — `months_waiting` is the average wait of
households who moved in, so a programme that admits nobody reports a short one. No rent burden and
no comparison with unsubsidized renters. No addresses and no names; the smallest geography published
is the census tract, and cells below a threshold are suppressed rather than rounded.
