---
name: Ohio county elected officials lookup — Allen County
description: >-
  The Ohio boards of elections' own roster of every elected official of a county, with party
  and with the exact day each term begins and ends. It is the first source in this corpus that
  dates a present-day office holding to the day, and the first that covers the county's elected
  offices as a set rather than one at a time.
type: dataset
obtained: true
retrieved: 2026-08-30
ttl_days: 180
location:
  - kind: url
    value: https://lookup.boe.ohio.gov/vtrapp/allen/cnm.aspx?task=voter&prsid=0004__1
    description: Allen County's elected officials, by office, with party and term start and end dates
used-by:
  - ../corpus/jurisdiction/allen-county-government.yml
  - ../corpus/office/allen-county-auditor.yml
  - ../corpus/office/allen-county-board-of-commissioners.yml
  - ../corpus/office/allen-county-clerk-of-courts.yml
  - ../corpus/office/allen-county-coroner.yml
  - ../corpus/office/allen-county-engineer.yml
  - ../corpus/office/allen-county-prosecuting-attorney.yml
  - ../corpus/office/allen-county-recorder.yml
  - ../corpus/office/allen-county-sheriff.yml
  - ../corpus/office/allen-county-treasurer.yml
  - ../corpus/person/beth-a-seibert.yml
  - ../corpus/person/brian-winegardner.yml
  - ../corpus/person/brion-e-rhodes.yml
  - ../corpus/person/cory-alan-noonan.yml
  - ../corpus/person/destiny-rae-caldwell.yml
  - ../corpus/person/jennifer-moree-mcbride.yml
  - ../corpus/person/john-thomas-meyer.yml
  - ../corpus/person/krista-n-bohn.yml
  - ../corpus/person/mona-s-losh.yml
  - ../corpus/person/rachael-s-gilroy.yml
  - ../corpus/tenure/auditor-2023-rachael-s-gilroy.yml
  - ../corpus/tenure/clerk-of-courts-2025-jennifer-moree-mcbride.yml
  - ../corpus/tenure/commissioner-2023-brian-winegardner.yml
  - ../corpus/tenure/commissioner-2025-beth-a-seibert.yml
  - ../corpus/tenure/commissioner-2025-cory-alan-noonan.yml
  - ../corpus/tenure/coroner-2025-john-thomas-meyer.yml
  - ../corpus/tenure/engineer-2025-brion-e-rhodes.yml
  - ../corpus/tenure/prosecutor-2025-destiny-rae-caldwell.yml
  - ../corpus/tenure/recorder-2025-mona-s-losh.yml
  - ../corpus/tenure/sheriff-2017-matthew-b-treglia.yml
  - ../corpus/tenure/treasurer-2025-krista-n-bohn.yml
---

**It closes the corpus's largest structural gap.** Before it, the `office` class held two instances —
[the sheriff](../corpus/office/allen-county-sheriff.yml) and the mayor of Lima — against the nine
elected county offices Ohio gives every county. This source names all nine, with the holder, the
party and the term dates of each. [verified]

**Nine offices, one party.** Every one of the nine county officers and all four common pleas judges
is recorded (R). [verified] The corpus does not read that as a fact about the county's voters
without an election return beside it; see [the returns](openelections-ohio.md).

**Its term dates are day-precision and they are not uniform**, which is the part a roster of names
would have hidden:

      commissioners     1/1/2023–12/31/2026 · 1/2/2025–1/1/2029 · 1/3/2025–1/2/2029
      auditor           3/13/2023–3/7/2027
      the other seven   1/6/2025–12/31/2028, except the treasurer at 9/1/2025–9/2/2029
      judges            six-year terms, three of the four beginning in February

Three consecutive days in January for three commissioners looks like a data-entry artifact and is
not one: [the statute](ohio-revised-code.md) assigns the first, second and third day of January to
distinguish the seats. The auditor's March date is statutory in the same way. Both were read here
before the statute was read, and the statute predicted both.

**What it does not carry.** No prior holders, so it dates the current term and not the beginning of
service — [the sheriff](../corpus/office/allen-county-sheriff.yml) has held office since 2017 and
appears here with a term beginning in 2025. Both are true and they answer different questions. It
gives no biography, no age, no district, and nothing about appointed officials.
