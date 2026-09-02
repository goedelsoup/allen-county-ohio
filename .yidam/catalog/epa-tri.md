---
name: Toxics Release Inventory (EPA, via Envirofacts)
description: >-
  Every pound of listed toxic chemical that an Allen County facility has reported releasing since
  1987 — to the air, to surface water, into the ground, and down a deep injection well — by
  facility, by chemical and by year. The county's first environmental measure of what industry
  puts out rather than what it makes.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 365
location:
  - kind: url
    value: https://data.epa.gov/efservice/tri_facility/state_abbr/OH/county_name/ALLEN/tri_reporting_form/tri_release_qty/rows/0:9999/CSV
    description: >-
      The three-table join this corpus reads, paged ten thousand rows at a time: `tri_facility`
      filtered to Ohio and Allen, joined to `tri_reporting_form` and then to `tri_release_qty`.
      55,329 release rows on 5,038 forms from 47 facility identifiers under 45 names — Randall Bearings
      and American Trim each hold two, at two addresses — for reporting years 1987 through 2024.
      One page returned `502` and succeeded on retry, so a short page is a transport failure here
      and not an empty result — check the row count against the `count/JSON` form of the same
      query before reading anything.
  - kind: url
    value: https://data.epa.gov/efservice/tri_chem_info/rows/0:9999/CSV
    description: >-
      The chemical dictionary, and the file that makes a long series honest. `active_date` and
      `inactive_date` give each chemical's first and last reportable year; `unit_of_measure`
      separates the 860 chemicals reported in pounds from those reported in grams; `carc_ind`
      marks the carcinogens.
  - kind: url
    value: https://data.epa.gov/efservice/tri_facility/state_abbr/OH/tri_reporting_form/reporting_year/2024/tri_release_qty/rows/0:9999/CSV
    description: >-
      The same join for the whole state in one year, 47,245 rows, which is what a county rank
      costs. Ohio-wide across all years was not attempted.
used-by:
  - ../corpus/measure/allen-county-toxic-releases-1987-2024.yml
---

**This is a file of self-reported estimates, not of measurements.** A facility over a threshold
computes what it released and files a form; nobody weighs it. The early years show it in the
figures themselves — 43,000,000 pounds, 11,000,000, 4,400,000 — round to two significant figures
because that is the precision the estimate had. [verified] —
[the release table](https://data.epa.gov/efservice/tri_facility/state_abbr/OH/county_name/ALLEN/tri_reporting_form/tri_release_qty/rows/0:9999/CSV),
reporting years 1987–1993.

**The chemical list is a roll that changes, and the totals are not comparable across a change to
it.** Ammonium sulfate (solution) was reportable from 1987 to 1993 and is not on the list now;
Allen County reported 126 million pounds of it, all in three years, and it is 17.3 per cent of
everything the county has ever reported. Nitrate compounds were added in 1995 and are 11.8 million
pounds since. [verified] — same source, against `active_date` and `inactive_date` in the chemical
dictionary. See
[a revision that changes the roll](../decisions/a-revision-that-changes-the-roll.yml).

**Four release codes are historical and two are their replacements, and mixing them would double
count.** `LANDF8795` and `UNINJ8795` appear only for reporting years 1987–1995; `SURF IMP` ends in
2002 where `SI 5.5.3A` and `SI 5.5.3B` begin in 2003. No form in this county carries a nonzero
quantity under both an old code and its replacement, which was checked rather than assumed.
[verified] — same source, 120 forms with a nonzero `UNINJ8795` and none of them with a nonzero
`UNINJ I`.

**A quantity under a thousand pounds may be reported as a range instead of a number.** 1,506 rows
in this county carry a `release_range_code` and no `total_release`. This corpus excludes them from
every total and says so; at the largest range the file allows they could add at most 1.5 million
pounds across all thirty-eight years, against 743.6 million pounds reported as numbers. [verified]
— same source, counted here.

**The form carries names this corpus does not record.** Each has a certifying official, a public
contact and a technical contact, with telephone numbers and email addresses. Those are individuals,
and only the facilities, their parent companies and the one federal installation are named here.
[verified] — `certif_name`, `asgn_public_contact` and `asgn_technical_contact` in the form table.
See
[what a tract page may be quoted for](../decisions/what-a-tract-page-may-be-quoted-for.yml).

**No second publication of these county totals was found.** EPA's own basic-data-file downloads
answer 404 or 500 at every path tried, so the figures in the node are this corpus's sum over the
release table and have no outside witness. What was checked instead is internal: no form–medium–
water-sequence key appears twice, and the five facility–chemical–year combinations carrying two
forms carry zero pounds between them. [verified] — computed here over all 55,329 rows.
