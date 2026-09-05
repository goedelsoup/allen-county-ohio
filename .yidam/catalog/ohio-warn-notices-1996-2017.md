---
name: Ohio WARN notices, 1996–2017 (Ohio Department of Job and Family Services)
description: >-
  Twenty-two annual registers of every plant closing and mass layoff an employer notified the
  State of Ohio about, with the company, the town, the number of workers, the layoff date and the
  union. It is the first source this corpus holds that records industrial decline as a sequence of
  dated events with names on them rather than as a line on a chart.
type: document
obtained: true
retrieved: 2026-09-04
location:
  - kind: url
    value: https://web.archive.org/web/20181015082630id_/http://jfs.ohio.gov/warn/Archived_Notices.stm
    description: >-
      The index, and the only complete list of the yearly files. It links 1996 through 2017 under
      six different filename conventions — `WARN_1996.PDF`, `warn_2001.pdf`, `Warn_2006.pdf`,
      `WARN_2007.stm`, `WARN2014.stm`, `WARN-2016.stm` — so the set cannot be enumerated by
      guessing. Snapshots of this page after 2019 no longer carry the year links.
  - kind: url
    value: https://web.archive.org/web/20161220231404id_/http://jfs.ohio.gov/warn/WARN_1996.PDF
    description: >-
      The earliest year, and the shape of the early files: `Date Rcd`, `Company`, `City`,
      `# Affected`, `Layoff Date`, `Phone Number`, `Union`, `WARN ID`. **City, not county** — see
      the caution below.
  - kind: url
    value: https://web.archive.org/web/20180102021909id_/http://jfs.ohio.gov/warn/WARN-2016.stm
    description: >-
      A late year, and the shape of the later files: the city column becomes `City (County)` from
      2008 and `City/County` from 2015. Files with a `.stm` extension from 2007 on are PDFs.
  - kind: url
    value: https://jfs.ohio.gov/warn/
    description: >-
      Where this lived. It answers HTTP 404 to every client, as does every path under it; the
      agency's later home at `dew.ohio.gov` does not resolve at all. Every file here was read from
      the Internet Archive, at the timestamps recorded in the nodes' method blocks.
used-by:
  - ../corpus/measure/allen-county-layoff-notices-1996-2017.yml
  - ../corpus/measure/allen-county-layoff-notice-intervals-1996-2017.yml
  - ../corpus/measure/allen-county-unions-in-layoff-notices-1996-2017.yml
---

**What the Act requires and what the register records.** The Worker Adjustment and Retraining
Notification Act obliges most employers of 100 or more to give sixty days' notice of a plant
closing or mass layoff — to the affected workers, to their representative, and to the state's
dislocated-worker unit. What this register records is the third of those: the date Ohio's Rapid
Response Section marked a notice received. It does not record when the employer wrote it, when it
was posted, or when anybody on the floor was told. [verified] — the index page's own description
of the Act, and the files' own column headings.

**The city column is not a county column, and in this county that matters twice.** Until 2008 the
files name a town and no county at all. Two of Allen County's places sit in two counties each —
Delphos in Allen and Van Wert, Bluffton in Allen and Hancock — so eight of the twenty-three
notices read here name a town whose county line runs through it. From 2008 the file adds the
county in brackets, and two of the eight Delphos rows are marked `(Allen)` by the state itself.
The other six are not attributable from this source; see
[a postal address is not a municipality](../decisions/a-postal-address-is-not-a-municipality.yml).

**The caution that produced a false hit.** Searching these files for Allen County place names
matches `Montgomery` on `Gomer`, `North Lima` on `Lima`, `Challenge Industries` on `Allen`, and
`Allen Telecom` of Solon on both. Every row published from here was read whole and by eye.

**Rows wrap and the company name is what wraps.** In the multi-column PDFs a long company name is
printed on the line above its own row, so a line-oriented read attributes
`Spencerville Metal Systems, LLC` to the previous notice and leaves this one as `Systems, LLC`.
Three of the twenty-three names here were recovered from the line above.

**The phone column is a join key.** The number is the employer's contact for the notice, and it
repeats: Harvard Industries of Spencerville in 1999 and Spencerville Metal Systems in 2006 both
give (419) 647-4101, which is how one plant laid off twice under two owners was found in a file
that has no identifier for a place of work. [verified] — the two rows.

**What is not here.** Nothing before 1996 and nothing after 2017: the index's last snapshot with
year links is from October 2018, and no yearly file for 2018 or later is archived under any name
tried. Ohio's later notices survive as individual employer letters under `/warn/pdf/`, which carry
no county and cannot be enumerated. Nothing below the Act's thresholds is here either — a layoff
of thirty people at a plant of ninety is not a WARN event and leaves no trace in this register.
