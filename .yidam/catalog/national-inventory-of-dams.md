---
name: National Inventory of Dams
description: >-
  The Corps of Engineers' register of every dam in the country above a size or hazard threshold,
  with its purposes, its storage, its hazard class and the date somebody last inspected it. Allen
  County has twelve rows in it and none of them is for flood control.
type: dataset
obtained: true
retrieved: 2026-09-07
ttl_days: 365
location:
  - kind: url
    value: https://nid.sec.usace.army.mil/api/nation/csv
    description: >-
      The whole inventory as one CSV — 92,766 rows, 64 MB, no key. The first line is not the
      header: it reads `Data Last Updated:,2026-8-28`, and the header is the second line.
  - kind: url
    value: https://nid.sec.usace.army.mil/
    description: The public map and the field documentation.
used-by:
  - ../corpus/measure/allen-county-dams-2026.yml
---

## What the corpus takes from it

`Purposes` is a semicolon-joined list from a closed vocabulary, and **Flood Risk Reduction** is one
of its values. That makes the absence of a purpose a checkable statement rather than an impression:
19.3 per cent of the nation's dams and 10.3 per cent of Ohio's 1,402 carry a flood purpose, and
none of Allen County's twelve does.

`Hazard Potential Classification` — Low, Significant, High, Undetermined — is **not a condition
rating**. It states what would happen downstream if the dam failed, so a new dam above houses is
High and a derelict dam above a field is Low. `Condition Assessment` is the separate judgement, and
it carries its own date. Reading the first as the second would call the county's newest and largest
reservoir one of its most dangerous structures.

## The trap in the county filter

`County` holds a bare name with no state FIPS beside it, so a substring match on `Allen` finds
Allen Parish in Louisiana and Allen County in Indiana and Kansas. Filter on `State` first and
compare `County` exactly. Within Ohio the string `Allen` matches nothing else.

## What it does not hold

**Low-head dams.** The seven on the Ottawa River in and above Lima — and the remnant of an eighth
— appear in none of the twelve rows, because a weir a few feet high impounding a headwaters
channel is below the inventory's threshold. A search of this file for the county's dams returns an
answer about its reservoirs. See
[the Ohio EPA study](ohio-epa-ottawa-river-tmdl.md), which recommended three of them for removal,
and [the county's Ottawa River service](allen-county-ottawa-river-service.md), which maps them one
by one.
