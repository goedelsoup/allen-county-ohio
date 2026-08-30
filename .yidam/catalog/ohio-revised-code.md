---
name: Ohio Revised Code
description: >-
  Ohio's codified statutes, as published by the Legislative Service Commission. The corpus uses it
  for the structure of county government — how many people hold an office, for how long, from what
  day, and what a holder must be.
type: reference
obtained: true
retrieved: 2026-08-30
ttl_days: 365
location:
  - kind: url
    value: https://codes.ohio.gov/ohio-revised-code/section-305.01
    description: Board of county commissioners — three members, four-year terms, staggered commencement
  - kind: url
    value: https://codes.ohio.gov/ohio-revised-code/section-313.02
    description: County coroner — must be a licensed physician in good standing
  - kind: url
    value: https://codes.ohio.gov/ohio-revised-code/section-315.02
    description: County engineer — must be a registered professional engineer and a registered surveyor
  - kind: url
    value: https://codes.ohio.gov/ohio-revised-code/section-319.01
    description: County auditor — chosen quadrennially, term begins the second Monday in March
  - kind: url
    value: https://codes.ohio.gov/ohio-revised-code/section-1901.02
    description: Municipal court territorial jurisdiction — the Lima court's is the whole county
used-by:
  - ../corpus/jurisdiction/lima-municipal-court.yml
  - ../corpus/office/allen-county-auditor.yml
  - ../corpus/office/allen-county-board-of-commissioners.yml
  - ../corpus/office/allen-county-clerk-of-courts.yml
  - ../corpus/office/allen-county-coroner.yml
  - ../corpus/office/allen-county-engineer.yml
  - ../corpus/office/allen-county-recorder.yml
  - ../corpus/office/allen-county-treasurer.yml
  - ../corpus/office/judge-of-the-lima-municipal-court.yml
  - ../corpus/person/brion-e-rhodes.yml
  - ../corpus/person/john-thomas-meyer.yml
  - ../corpus/tenure/auditor-2023-rachael-s-gilroy.yml
  - ../corpus/tenure/commissioner-2025-beth-a-seibert.yml
  - ../corpus/tenure/commissioner-2025-cory-alan-noonan.yml
---

**It is a source about the county that never mentions the county.** Everything it says here is true
of all eighty-eight Ohio counties, and the corpus records it on Allen County's offices because that
is where a reader will look for it. Nothing drawn from it distinguishes this county from any other,
and no claim resting on it alone should be read as one.

**Four sections, read in full:**

> §305.01 — "The board of county commissioners shall consist of three persons" … "such officers
> shall hold office for the term of four years and until their successors are elected and
> qualified." One seat's term begins on the first day of January after election; of the two elected
> together, one begins on "the second day of January next after his election" and one on "the third
> day".

> §313.02 — a coroner must be "a physician who is licensed under Chapter 4731. of the Revised Code
> to practice medicine and surgery or osteopathic medicine and surgery, and who is in good standing
> in the person's profession", holding that licence for at least two years before election.

> §315.02 — a county engineer must be "a registered professional engineer and a registered
> surveyor, licensed to practice in this state". And: "No person holding the office of clerk of the
> court of common pleas, sheriff, county treasurer, or county recorder is eligible to hold the
> office of county engineer."

> §319.01 — the auditor is chosen "quadrennially in each county", holds office "for four years",
> and the term begins "on the second Monday in March next after his election."

**Two of these were tested against an independent source and passed.** The three commissioners'
term dates and the auditor's March date were read from
[the elections roster](ohio-boe-elected-officials-allen.md) before these sections were fetched, and
each statutory rule predicts the dates already in hand — 1 January, 2 January and 3 January for the
three commissioners, and 13 March 2023 for the auditor, which is the second Monday of that March.

> §1901.02 — municipal courts "have jurisdiction within the corporate limits of their respective
> municipal corporations", and then, among the named exceptions: "The Lima municipal court has
> jurisdiction within Allen county."

That last one is the only section here that says something true of this county and not of the other
eighty-seven, and it is the section that stopped the corpus writing that a court called after Lima
serves Lima. See [the name is the seat, not the extent](../decisions/the-name-is-the-seat-not-the-extent.yml).

**What it is not used for here.** The corpus has not read the chapters establishing the treasurer,
recorder, prosecutor or clerk of courts, so those offices carry their term length as arithmetic on
the roster's dates rather than as statute.
