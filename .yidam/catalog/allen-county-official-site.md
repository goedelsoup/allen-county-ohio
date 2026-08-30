---
name: Allen County, Ohio — county government website
description: >-
  The county commissioners' own public site. It is the first source in this corpus that says
  anything about how the county is governed rather than where its lines run — township houses,
  boards of trustees, meeting schedules, and the years its municipalities were founded and
  incorporated.
type: other
obtained: true
retrieved: 2026-08-29
ttl_days: 365
location:
  - kind: url
    value: https://allencountyohio.com/townships/
    description: All 12 townships — township house address, telephone, trustee meeting schedule
  - kind: url
    value: https://allencountyohio.com/cities-villages/
    description: Both cities and all 7 villages — founding and incorporation years, name origins
  - kind: url
    value: https://allencountyohio.com/
    description: Index, and the linked sites of the nine elected county offices
used-by:
  - ../corpus/jurisdiction/amanda-township.yml
  - ../corpus/jurisdiction/american-township.yml
  - ../corpus/jurisdiction/auglaize-township.yml
  - ../corpus/jurisdiction/bath-township.yml
  - ../corpus/jurisdiction/city-of-delphos.yml
  - ../corpus/jurisdiction/jackson-township.yml
  - ../corpus/jurisdiction/marion-township.yml
  - ../corpus/jurisdiction/monroe-township.yml
  - ../corpus/jurisdiction/perry-township.yml
  - ../corpus/jurisdiction/richland-township.yml
  - ../corpus/jurisdiction/shawnee-township.yml
  - ../corpus/jurisdiction/spencer-township.yml
  - ../corpus/jurisdiction/sugar-creek-township.yml
  - ../corpus/jurisdiction/village-of-beaverdam.yml
  - ../corpus/jurisdiction/village-of-cairo.yml
  - ../corpus/jurisdiction/village-of-elida.yml
  - ../corpus/jurisdiction/village-of-harrod.yml
  - ../corpus/jurisdiction/village-of-lafayette.yml
  - ../corpus/jurisdiction/village-of-spencerville.yml
  - ../corpus/office/allen-county-auditor.yml
  - ../corpus/office/allen-county-board-of-commissioners.yml
  - ../corpus/office/allen-county-clerk-of-courts.yml
  - ../corpus/office/allen-county-coroner.yml
  - ../corpus/office/allen-county-engineer.yml
  - ../corpus/office/allen-county-prosecuting-attorney.yml
  - ../corpus/office/allen-county-recorder.yml
  - ../corpus/office/allen-county-treasurer.yml
  - ../corpus/person/brion-e-rhodes.yml
  - ../corpus/person/john-thomas-meyer.yml
  - ../corpus/person/mona-s-losh.yml
  - ../corpus/place/beaverdam.yml
  - ../corpus/place/bluffton.yml
  - ../corpus/place/cairo.yml
  - ../corpus/place/delphos.yml
  - ../corpus/place/elida.yml
  - ../corpus/place/harrod.yml
  - ../corpus/place/lafayette.yml
  - ../corpus/place/lima.yml
  - ../corpus/place/spencerville.yml
  - ../corpus/site/miami-and-erie-canal.yml
  - ../corpus/tenure/coroner-2025-john-thomas-meyer.yml
  - ../corpus/tenure/engineer-2025-brion-e-rhodes.yml
  - ../corpus/tenure/recorder-2025-mona-s-losh.yml
---

**What it settles.** Three phases of this corpus deferred the same thing: the `jurisdiction`
side of the county's 21 civil divisions, on the ground that no held source said a township had a
board of trustees or when a village incorporated. See
[the-civil-geography](../decisions/the-civil-geography.yml), which reasoned that writing 21 nodes
whose only content was "this presumably has a government" would collapse the distinction between
`place` and `jurisdiction`. The county's own site carries both, and it was never retrieved because
the retrievals had all been federal.

Its townships page gives, for each of the twelve, a township house with a street address, a
telephone number, and the days and hour the trustees meet. A body that meets on the second and
fourth Monday at 7:00 p.m. is a body. Its cities-and-villages page gives founding years for both
cities and all seven villages, incorporation years for three of them, and a name origin for four.

**What it is authoritative for, and what it is not.** This is the county government publishing its
own administrative arrangements, and for those — which townships exist, where their houses are,
when their boards meet — there is no better source. It is **not** a historical source. Not one of
the founding or incorporation years carries a citation, and the page uses "established",
"founded" and "incorporated" without holding them apart:

    Beaverdam    "established in 1853 ... incorporated in 1878"     both, and distinguished
    Elida        "established in 1852 and incorporated in 1878"     both, and distinguished
    Lafayette    "incorporated in 1868"                             corporation only
    Harrod       "the village was established in 1884"              settlement only
    Spencerville "founded in 1844 on the banks of the Miami-Erie"   settlement only
    Cairo        "established in 1848 ... name changed in 1922"     settlement only
    Lima         "founded in 1831"                                  settlement only
    Delphos      "in 1851 the four towns merged"                    neither, exactly

That distinction is the one this corpus's ontology is built on — a settlement is a `place` and a
corporation is a `jurisdiction` — so the years are recorded on whichever class the wording
actually supports, and the rest is left open rather than guessed.

**It agrees with the parcel data, which is the check that matters.** Five of the twelve township
houses carry postal addresses naming a municipality in a different civil subdivision, and Monroe
Township's names one in **Putnam County**. The county's own address file locates eleven of the
twelve by coordinate with a blank `MUNI` — the county's answer for "in no municipality" — and
point-in-polygon against [TIGERweb](tigerweb-census2020.md) puts every one of the eleven inside
its own township. Two county products and one federal one agree, and the postal addresses are
postal. See [a-postal-address-is-not-a-municipality](../decisions/a-postal-address-is-not-a-municipality.yml).

**What it does not carry.** No trustee names, no township erection dates, no municipal charters or
officers, no budgets, and no election returns. The Ohio Secretary of State was probed again on
2026-08-29 and still answers 403 to an automated client, on both its municipalities publication
and its election-results index.
