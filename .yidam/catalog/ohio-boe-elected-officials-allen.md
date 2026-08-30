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
  - ../corpus/division/ohio-house-district-4-2020.yml
  - ../corpus/division/ohio-house-district-78-2023.yml
  - ../corpus/division/ohio-senate-district-12-2020.yml
  - ../corpus/division/ohio-senate-district-12-2023.yml
  - ../corpus/jurisdiction/allen-county-court-of-common-pleas.yml
  - ../corpus/jurisdiction/allen-county-government.yml
  - ../corpus/jurisdiction/lima-municipal-court.yml
  - ../corpus/office/allen-county-auditor.yml
  - ../corpus/office/allen-county-board-of-commissioners.yml
  - ../corpus/office/allen-county-clerk-of-courts.yml
  - ../corpus/office/allen-county-coroner.yml
  - ../corpus/office/allen-county-engineer.yml
  - ../corpus/office/allen-county-prosecuting-attorney.yml
  - ../corpus/office/allen-county-recorder.yml
  - ../corpus/office/allen-county-sheriff.yml
  - ../corpus/office/allen-county-treasurer.yml
  - ../corpus/office/judge-of-the-common-pleas-domestic-relations-division.yml
  - ../corpus/office/judge-of-the-common-pleas-general-division.yml
  - ../corpus/office/judge-of-the-common-pleas-probate-division.yml
  - ../corpus/office/judge-of-the-lima-municipal-court.yml
  - ../corpus/office/judge-of-the-third-district-court-of-appeals.yml
  - ../corpus/office/member-of-the-ohio-house-for-district-78.yml
  - ../corpus/office/member-of-the-ohio-senate-for-district-12.yml
  - ../corpus/office/representative-in-congress-for-ohios-4th-district.yml
  - ../corpus/person/beth-a-seibert.yml
  - ../corpus/person/brian-winegardner.yml
  - ../corpus/person/brion-e-rhodes.yml
  - ../corpus/person/cory-alan-noonan.yml
  - ../corpus/person/destiny-rae-caldwell.yml
  - ../corpus/person/james-d-jordan.yml
  - ../corpus/person/jeffrey-l-reed.yml
  - ../corpus/person/jennifer-moree-mcbride.yml
  - ../corpus/person/john-r-willamowski.yml
  - ../corpus/person/john-richard-payne.yml
  - ../corpus/person/john-thomas-meyer.yml
  - ../corpus/person/juergen-a-waldick.yml
  - ../corpus/person/krista-n-bohn.yml
  - ../corpus/person/mark-c-miller.yml
  - ../corpus/person/matt-c-staley.yml
  - ../corpus/person/matthew-c-huffman.yml
  - ../corpus/person/mona-s-losh.yml
  - ../corpus/person/rachael-s-gilroy.yml
  - ../corpus/person/susan-manchester.yml
  - ../corpus/person/tammie-k-hursh.yml
  - ../corpus/person/terri-lynn-kohlrieser.yml
  - ../corpus/person/todd-e-kohlrieser.yml
  - ../corpus/person/william-r-zimmerman.yml
  - ../corpus/place/allen-county.yml
  - ../corpus/question/allen-county-current-congressional-district.yml
  - ../corpus/tenure/auditor-2023-rachael-s-gilroy.yml
  - ../corpus/tenure/clerk-of-courts-2025-jennifer-moree-mcbride.yml
  - ../corpus/tenure/commissioner-2023-brian-winegardner.yml
  - ../corpus/tenure/commissioner-2025-beth-a-seibert.yml
  - ../corpus/tenure/commissioner-2025-cory-alan-noonan.yml
  - ../corpus/tenure/congress-2025-james-d-jordan.yml
  - ../corpus/tenure/coroner-2025-john-thomas-meyer.yml
  - ../corpus/tenure/domestic-2023-matt-c-staley.yml
  - ../corpus/tenure/engineer-2025-brion-e-rhodes.yml
  - ../corpus/tenure/general-2023-jeffrey-l-reed.yml
  - ../corpus/tenure/general-2025-terri-lynn-kohlrieser.yml
  - ../corpus/tenure/municipal-2022-tammie-k-hursh.yml
  - ../corpus/tenure/municipal-2024-john-richard-payne.yml
  - ../corpus/tenure/ohio-house-2025-matthew-c-huffman.yml
  - ../corpus/tenure/ohio-senate-2025-susan-manchester.yml
  - ../corpus/tenure/probate-2021-todd-e-kohlrieser.yml
  - ../corpus/tenure/prosecutor-2025-destiny-rae-caldwell.yml
  - ../corpus/tenure/recorder-2025-mona-s-losh.yml
  - ../corpus/tenure/sheriff-2017-matthew-b-treglia.yml
  - ../corpus/tenure/third-district-2021-mark-c-miller.yml
  - ../corpus/tenure/third-district-2023-juergen-a-waldick.yml
  - ../corpus/tenure/third-district-2023-william-r-zimmerman.yml
  - ../corpus/tenure/third-district-2025-john-r-willamowski.yml
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

**It reaches far past the county, and the corpus reads only part of it.** The roster is organized
from the President down: federal offices, statewide executives, the Supreme Court, the court of
appeals, state legislators, county officers, municipal officers, township trustees and school
boards. This corpus takes from it the officers whose constituency is smaller than the state and
includes this county's ground, and records the rest here rather than as nodes. [verified]

The statewide and national officers it names, recorded once and not as nodes because they are
identical for all eighty-eight Ohio counties: Donald J. Trump as President; Bernie Moreno and Jon
Husted (appointed) as United States Senators; Mike DeWine as Governor, Dave Yost as Attorney
General, Keith Faber as Auditor of State, Frank LaRose as Secretary of State and Robert Sprague as
Treasurer of State; Sharon L. Kennedy as Chief Justice with Justices Megan E. Shanahan, Joseph T.
Deters, Daniel R. Hawkins, Jennifer Brunner, Pat Fischer and Pat DeWine; and Kristie Reighard on
the State Board of Education. [verified]

**One party, with a single exception, and the exception is statewide.** Every officer on this
roster whose constituency includes Allen County and is smaller than Ohio is recorded (R) — nine
county officers, six judges of two county courts, four appellate judges, a state senator and a
state representative. The only (D) anywhere on it is Justice Jennifer Brunner, elected statewide.
[verified] The corpus does not read that as a fact about this county's voters without an election
return beside it; see [the returns](openelections-ohio.md).

**What it does not carry.** No prior holders, so it dates the current term and not the beginning of
service — [the sheriff](../corpus/office/allen-county-sheriff.yml) has held office since 2017 and
appears here with a term beginning in 2025. Both are true and they answer different questions. It
gives no biography, no age, no district, and nothing about appointed officials.
