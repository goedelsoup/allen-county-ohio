---
name: City of Lima official website
description: >-
  The municipal government's own site. It is the corpus's first source published by the city rather
  than about it, and the only one that names the city's departments, its appointed officers and the
  existence of its charter.
type: website
obtained: true
retrieved: 2026-09-01
ttl_days: 180
location:
  - kind: url
    value: https://www.limaohio.gov/98/City-Council
    description: >-
      The council page. Names all seven ward members and the president in a photograph caption,
      names the clerk of council, and links the ward map, the council rules and the charter. The
      address cataloged here at genesis, `www.cityhall.lima.oh.us`, now returns 301 to
      `www.limaohio.gov` and the page numbers are unchanged, so every path recorded here still
      resolves.
  - kind: url
    value: https://codelibrary.amlegal.com/codes/lima/latest/lima_oh/0-0-0-17317
    description: >-
      "Lima City Charter and Code", at American Legal Publishing. **Not obtained.** Returns 403 to
      an automated client with or without a browser user agent, which is a Cloudflare refusal rather
      than a missing document.
  - kind: url
    value: https://www.limaohio.gov/117/Mayor
    description: >-
      The mayor's page. Names the sitting mayor, dates her first election to 2021, says she is
      serving her second term, and calls her the first woman and first African American elected to
      lead the city. It names no predecessor, and the site's sitemap carries no list of former
      mayors; the only "History" page on it is the fire department's.
used-by:
  - ../corpus/jurisdiction/city-of-lima.yml
  - ../corpus/office/mayor-of-lima.yml
  - ../corpus/person/derry-l-glenn.yml
  - ../corpus/person/jamie-lamar-dixon-jr.yml
  - ../corpus/person/jesse-james-lowe-ii.yml
  - ../corpus/person/jonathan-m-neeper.yml
  - ../corpus/person/thomas-michael-wa-jones.yml
  - ../corpus/tenure/lima-council-5th-ward-2026-thomas-michael-wa-jones.yml
  - ../corpus/tenure/mayor-2025-sharetta-t-smith.yml
---

**It establishes that Lima has a charter**, which the corpus had carried as an inference since
genesis and as an explicit `[open]` on [the mayor's office](../corpus/office/mayor-of-lima.yml).
The council page links the document by name. It does not serve it. [verified]

**It names an officer no roster of elected officials can.** Dana Addis is clerk of council, and the
council page gives the clerk's telephone number as the way to request privilege of the floor. A
board-of-elections roster carries only elected seats, so an appointed officer of a legislative body
is invisible to it. [verified]

**Its council page confirms the roster's seven wards from the other side**, naming Jesse Lowe
(Third), Thomas Jones (Fifth), Jon Neeper (Seventh), Derry Glenn (Sixth), Todd Gordon (First),
Anthony Wilkerson (Second), Jamie Dixon (President) and Jeannine Jordan (Fourth) — the same eight
people the [elected officials lookup](ohio-boe-elected-officials-allen.md) gives, in a caption
rather than a table. Two sources of very different kinds agreeing on eight names is the cheapest
confirmation available that the roster was read correctly. [verified]

**What is on it and unread.** The departments index, the mayor's page, agendas and minutes of
council back some years, a ward map as PDF, the council rules, and the committee structure. None of
it has been retrieved.

**The domain moved and the corpus found out by following a redirect.** Everything cataloged here at
genesis was under `cityhall.lima.oh.us`. That host now answers 301 to `limaohio.gov` with the same
page numbers, so nothing was lost — but a `ttl_days` of 180 would not have caught it, because a
redirect is not a failure. What caught it was asking the site a new question.

**What it will not answer.** Who was mayor before the sitting one. The mayor's page names her alone,
the sitemap has no roster of predecessors, and the city's only history page is the fire department's.
The corpus's line of mayors therefore ends in 1922 and resumes in 2021, and the gap is not for want
of asking the city.
