---
name: EPA Toxics Release Inventory, via Envirofacts
description: >-
  Every US industrial facility that has reported a listed chemical since 1987, with its address, its
  corporate parent, one form per chemical per year, and the quantities on those forms. It is the
  only source this corpus has that names the county's manufacturers as a set, the only one that
  says how long each has been here, and the only one that says how much each puts out.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 365
location:
  - kind: url
    value: https://data.epa.gov/efservice/tri_facility/county_name/ALLEN/state_abbr/OH/CSV
    description: >-
      The facility register. 49 rows for Allen County — the facilities that have ever reported,
      with current name, address, coordinates, closure flag and parent company.
  - kind: url
    value: https://data.epa.gov/efservice/tri_facility/county_name/ALLEN/state_abbr/OH/tri_reporting_form/CSV
    description: >-
      The register joined to the forms. 5,372 rows for Allen County across reporting years 1987 to
      2024. Envirofacts pages at 1,000 rows; append `/rows/0:999` and step.
---

**Twenty facilities reported for 2024 and the register holds forty-nine.** Twenty-eight of the
forty-eight that have ever filed did not file for 2024, and three of those twenty-eight carry the
closure flag. [verified] The flag catches about one departure in nine, so it is not a currency
test and this corpus does not use it as one. What answers *who is here now* is whether a facility
filed a form for the latest year. See
[a register is not a census](../decisions/a-register-is-not-a-census.yml) and
[the reporters](../corpus/measure/allen-county-tri-reporters-1987-2024.yml).

**Its facility attributes are current and its forms are historical, and joining them is a trap.**
`parent_co_name` and `facility_name` live in the register and describe today. Joined to a 1987 form
they print today's owner against a thirty-eight-year-old filing. The Lima refinery's 1987 row reads
`CENOVUS ENERGY INC.`, and [EIA](eia-refinery-capacity.md) has BP operating that refinery until
1997 and Cenovus not appearing until 2022. [verified] Every parent named from this source is a
parent *now*.

**Its coordinates must not be used for containment.** They are stated with an accuracy field, and
the field is 150 metres for the refinery and the Ford engine plant and **11,000 metres** for the
Joint Systems Manufacturing Center. [verified] The refinery's point falls 0.96 miles west of the
one [the county's own address file](allen-county-gis-downloads.md) gives, and the two land in
different jurisdictions. The corpus takes the county's point, which is address-matched by the
authority that maintains the addresses, and does not ask this source where anything is.

**Two records at one address, and the difference is a reporter and not an operator.** The tank plant
appears twice: `U.S. ARMY JSMC GENERAL DYNAMICS LAND SYSTEMS`, parent General Dynamics Corp,
reporting 1987 to 1993; and `U S ARMY JOINT SYSTEMS MANUFACTURING CENTER`, parent US Department of
Defense, reporting 1994 to 2024. Same street address, same coordinates. [verified] Who filed
changed in 1994. Whether who *operated* changed, this source cannot say. [open]

**The quantities are now read, and the transfers are not.** What this corpus has taken from the
release table is on-site releases per chemical per year, by medium. Off-site transfers, waste
management and source reduction are still unread, and a facility that ships its waste elsewhere
rather than releasing it on site does not appear in any figure here.

**It is a file of self-reported estimates, not of measurements.** A facility over a threshold
computes what it released and files the number; nobody weighs it. The early years show the
precision in the figures themselves — 43,000,000 pounds, 11,000,000, 4,400,000 — round to two
significant figures because that is what the estimate was worth. [verified] — the release table,
reporting years 1987–1993.

**The chemical list is a roll that changes, and totals are not comparable across a change to it.**
Ammonium sulfate (solution) was reportable from 1987 to 1993 and is not on the list now; this county
reported 126 million pounds of it, all in three years, and it is 17.3 per cent of everything the
county has ever reported. Nitrate compounds were added in 1995 and are 11.8 million pounds since.
[verified] — same source, against `active_date` and `inactive_date` in the chemical dictionary. See
[a revision that changes the roll](../decisions/a-revision-that-changes-the-roll.yml).

**Four release codes are historical and two are their replacements, and mixing them would double
count.** `LANDF8795` and `UNINJ8795` appear only for reporting years 1987–1995; `SURF IMP` ends in
2002 where `SI 5.5.3A` and `SI 5.5.3B` begin in 2003. No form in this county carries a nonzero
quantity under both an old code and its replacement, which was checked rather than assumed.
[verified] — same source, 120 forms with a nonzero `UNINJ8795` and none of them with a nonzero
`UNINJ I`.

**A quantity under a thousand pounds may be reported as a range instead of a number.** 1,506 rows in
this county carry a `release_range_code` and no `total_release`. This corpus excludes them from
every total and says so; at the largest range the file allows they could add at most 1.5 million
pounds across all thirty-eight years, against 743.6 million reported as numbers. [verified] — same
source, counted here.

**Filing a form is not reporting a quantity, and three counts of "facilities" differ.** For 2024,
twenty facilities filed; seventeen of them appear in the release table; fifteen reported a nonzero
quantity in pounds. The three that filed without a release row filed the short certification form,
which states that a facility is under the threshold and carries no quantities, and two more reported
only ranges. [verified] — same source, against
[the reporters](../corpus/measure/allen-county-tri-reporters-1987-2024.yml).

**The form carries names this corpus does not record.** Each has a certifying official, a public
contact and a technical contact, with telephone numbers and email addresses. Those are individuals,
and only the facilities, their parent companies and the one federal installation are named here.
[verified] — `certif_name`, `asgn_public_contact` and `asgn_technical_contact` in the form table.
See [what a tract page may be quoted for](../decisions/what-a-tract-page-may-be-quoted-for.yml).

**No second publication of these county totals was found.** EPA's own basic-data-file downloads
answer 404 or 500 at every path tried, so the figures in the node are this corpus's sum over the
release table and have no outside witness. What was checked instead is internal: no form–medium–
water-sequence key appears twice, and the five facility–chemical–year combinations carrying two
forms carry zero pounds between them. [verified] — computed here over all 55,329 rows.
