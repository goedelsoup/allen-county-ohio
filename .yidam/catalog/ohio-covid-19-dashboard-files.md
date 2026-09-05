---
name: Ohio Department of Health COVID-19 dashboard files
description: >-
  The state's own line-summary files for the pandemic — every county, sex, age band and date, with
  cases by onset, hospitalisations by admission and deaths by date of death, published twice over:
  once by the decedent's county of residence and once by the county the death occurred in. Ohio
  took them off the web; they survive in the Internet Archive.
type: dataset
obtained: true
retrieved: 2026-09-05
ttl_days: 3650
location:
  - kind: url
    value: https://web.archive.org/web/20230903084228id_/https://coronavirus.ohio.gov/static/dashboards/COVIDDeathData_CountyOfResidence.csv
    description: >-
      3 September 2023, 37.6 MB, 945,836 rows. County, sex, age range, onset date, admission date,
      date of death, case count, hospitalised count and deaths by county of residence. This is the
      fullest of the three and the one every resident figure here comes from.
  - kind: url
    value: https://web.archive.org/web/20230503074254id_/https://coronavirus.ohio.gov/static/dashboards/COVIDDeathData_CountyOfDeath.csv
    description: >-
      3 May 2023, 39.5 MB, 916,571 rows. The same shape with two columns added — state of death and
      state of residence — and the death column counted by the county the death happened in. Paired
      with the residence file captured 98 seconds later on the same day, which is the pair used for
      every comparison below.
  - kind: url
    value: https://web.archive.org/web/20230807190443id_/https://coronavirus.ohio.gov/static/dashboards/COVIDSummaryDataZIP.csv
    description: >-
      Cumulative cases and cases per 100,000 for 1,189 Ohio ZIP codes, 19.7 KB. The only sub-county
      grain the state published.
  - kind: url
    value: https://web.archive.org/web/20230827103648id_/https://coronavirus.ohio.gov/static/dashboards/COVIDSummaryData.csv
    description: >-
      The file the dashboard was originally built on. Frozen — see below. Not used for any figure.
  - kind: url
    value: https://coronavirus.ohio.gov/static/dashboards/COVIDSummaryData.csv
    description: >-
      The live path, which now answers 404 at every one of these filenames. Nothing in this entry
      can be re-fetched from the state.
---

**What it is, and why the corpus has it at all.** Ohio published its pandemic dashboards as flat
CSVs at fixed URLs and retired them; every one of those URLs now returns a 404 page. The Internet
Archive holds 233 captures of that directory, including 37 of the residence file and 34 of the
county-of-death file. [verified] — the CDX index for `coronavirus.ohio.gov/static/dashboards/*`,
read here. This is the second time this corpus has recovered a retired state series from the
archive; the first was
[the WARN notices](ohio-warn-notices-1996-2017.md).

**One row is a combination and not a person.** The key is county, sex, age range and up to three
dates, and the three counts on a row are independent of each other: a row with an onset date and no
death date carries cases, a row with a date of death carries deaths and nothing else. Summing the
case column over rows that have a death date returns zero. [verified] — the files, read here.

**Its county column is padded on one block of rows, and the padded rows are the deaths that
happened out of state.** The residence file carries 1,336 rows whose county name is padded to a
fixed width — `Belmont` followed by twenty-one spaces — and those rows hold 1,344 deaths, no cases
and no hospitalisations. They are concentrated in the border counties: Lawrence 229, Belmont 193,
Jefferson 94, Trumbull 71. [verified] — the file, grouped on the raw string and on the stripped
one. A reader who groups on the raw string gets 171 counties for Ohio's 88 and loses 78 per cent of
Lawrence County's dead. Allen County loses two.

**Its totals.** Ohio, through 31 August 2023: 3,486,445 cases, 142,041 hospitalisations and 42,490
deaths of Ohio residents; 42,051 deaths occurring in Ohio through 3 May 2023, of which 1,211 were
of people who lived in another state — 268 West Virginians, 238 Michiganders, 194 Kentuckians and
168 Indianans. [verified] — the two files, summed here.

**The residence file and the death file are the same measurement twice and they do not have to
agree county by county.** At the state line they nearly do: 42,168 resident deaths against 42,051
deaths in Ohio on the same day. Inside the state they diverge by a factor of twelve — Allen County
records 815 deaths and has 513, Lawrence County records 39 and has 291. [verified] — the matched
pair of 3 May 2023. See
[located here is not of here](../decisions/located-here-is-not-of-here.yml).

**`COVIDSummaryData.csv` is still served, still archived, and stopped being written in March 2021.**
Six captures between 13 September 2021 and 27 August 2023 are byte-identical — the same MD5, the
same 263,145 rows, the same Ohio grand total of 968,874 cases — and the latest onset date in every
one of them is **1 March 2021**. [verified] — the six, downloaded and compared here. Nothing on the
page, in the filename or in the archive says so; the only tell is the maximum of its own date
column. See [a live url is not a live file](../decisions/a-live-url-is-not-a-live-file.yml).

**The ZIP file's denominator is not the census's.** It publishes a population beside each ZIP code
and those populations differ from the 2020 census count of the same tabulation area by up to seven
and a half per cent — 15,665 against 14,573 for Lima's 45804. Rates quoted from this file are on
its own denominator and are not comparable with a rate this corpus computes. [verified] — the file
against [the ZIP codes](../corpus/measure/allen-county-zip-codes-2020.yml).

**What it does not carry.** No race, no ethnicity, no census tract, no facility, no vaccination
status, no cause other than the one it is about, and no way to tell a nursing-home death from a
hospital one. The age bands are eight and the youngest is 0–19.
