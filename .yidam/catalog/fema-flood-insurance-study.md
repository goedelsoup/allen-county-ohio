---
name: FEMA Flood Insurance Study, Allen County, Ohio
description: >-
  The engineering report behind the county's flood maps. This corpus read it first as study
  39003CV000A, effective 2 May 2013, and that document has been superseded twice: the effective
  study is 39003CV001C, revised 20 June 2024. Beside the hydraulics and the discharge tables it
  carries a prose flood history — which floods this county has had, what the water did in Lima in
  January 1959, why the Ottawa backs up where it does — and it is the only enumerated list of this
  county's floods in any source the corpus has found. Its Tables 12 and 28 date every analysis the
  effective map rests on.
type: document
obtained: true
retrieved: 2026-09-07
ttl_days: 1825
location:
  - kind: url
    value: https://msc.fema.gov/portal/downloadProduct?productTypeID=FINAL_PRODUCT&productSubTypeID=FIS_REPORT&productID=39003CV001C
    description: >-
      The effective study, 3.7 MB, revised 20 June 2024. `map1.msc.fema.gov/data/39/S/PDF/` serves
      39003CV000A and 39003CV000B and returns 404 for every 001-series name, so the current report
      is reachable only through the portal's product endpoint. Table 12 is the summary of hydrologic
      and hydraulic analyses, with a date-of-analyses column; Table 27 is the community map history;
      Table 28 is the summary of contracted studies, with a work-completed column. Those two date
      columns are what this study says about its own age.
  - kind: url
    value: https://map1.msc.fema.gov/data/39/S/PDF/39003CV000A.pdf
    description: >-
      The countywide study text, 966 KB. Section 2.3 is the flood history, 2.4 the flood protection
      measures, 9.0 the bibliography — which is itself the finding-aid for the six superseded
      studies this one absorbed, from Fort Shawnee in 1984 to Bluffton in 1995. The FEMA Map Service
      Center serves it under a path keyed to the state and study number; it is not a stable citation
      across map revisions, and the effective date in the document is what dates it.
used-by:
  - ../corpus/event/the-ottawa-river-flood-of-1959.yml
  - ../corpus/event/the-flood-of-22-august-2007.yml
  - ../corpus/measure/allen-county-flood-map-currency-2026.yml
---

**What it is for and what it is therefore careless about.** The study exists to set base flood
elevations, and its flood history is background written to justify a hydraulic model. It cites
earlier studies rather than primary records, its references are numbered rather than dated in the
text, and it contains ordinary drafting errors — "the Ottawa River reached flood staged in several
areas". None of that touches the value of the passages below, which are the only account of these
floods anybody has written down where this corpus can reach it.

**Its January 1959 account is the one this corpus had been missing.** Verbatim: "In January 1959,
the Ottawa River reached flood staged in several areas within the study limits. Most of the area
between Schoonover Park and Thayer Road experienced varying degrees of flooding. During the flood
period, rain fell on snow and frozen ground, and the water rose rapidly upstream from the stone
quarry. An ice jam occurred at the Rousch Road Bridge aggravating already dangerous conditions.
During the night, the waters spilled over the dike portion of track and filling the sludge pit and
lowland south of the railroad. It has been estimated that 3,000 to 5,000 acre-feet of water
overflowed the channel at the break. The water immediately receded around the Rousch Road area after
the break. As a result, the river never reached a very high crest below the quarry."

**That last sentence is a caution about the gauge.** The Allentown gauge is downstream of the break,
so the 7,740 cubic feet a second it recorded on 22 January 1959 is what the channel still carried
after three to five thousand acre-feet had left it. The largest flow ever measured on this river is
a lower bound on the water that reached Lima.

**Its flood list is the county's only one.** Verbatim: "Other flood of lesser intensity occurred on
the Ottawa River in April 1904, December 1916, January 1937, March 1939, May 1943, June 1957,
February 1959 and April 1964. Smaller floods have occurred at more frequent intervals, with minor
flooding in lower areas occurring almost annually." Three of those eight — 1904, 1916 and 1937 —
lie outside every instrument record this corpus can reach.

**It names the county's largest flood and it is not the one the gauges rank first.** "The greatest
flood event on record in the City of Lima occurred in 2007 due to several rounds of heavy rainfall."
No gauge in Allen County was running that year.

**It said there is no flood control here, in words, and the effective study no longer says it.**
Section 2.4 of the 2013 report reads: "The only known flood protection measures are within the City
of Lima… There is no established maintenance program outside the corporate limits. Since levees,
dams, and control works are not economically feasible, it is important that the proper use and
development of the flood plain be guided by factual information concerning the flood history and
potential future floods. Similarly, protection against flooding is not provided along Pike Run or
Lost Creek. None of the dams along the different streams are used for flood control (Reference 5)."
The 2024 report has no such paragraph. In its place, Table 6 (historic flooding elevations), Table 7
(non-levee flood protection measures) and Table 8 (levees) each read, in full,
"[Not Applicable to this Flood Risk Project]", and §4.4 says only "This section is not applicable to
this Flood Risk Project." [verified] — both documents, read here. **The fact survives and its reason
does not**: a paragraph that said why there is no flood control became three empty tables that say
there is none.

**Reference 5 is a 1978 document, so the passage the corpus quotes was already thirty-five years old
when the corpus found it.** The 2013 bibliography's fifth entry is the Flood Insurance Study for the
City of Lima of August 1978, and the 2024 report attributes the same 1959 account and the same flood
list to "(FEMA, 1978)" in the running text. [verified] The county's only enumerated flood list is a
1978 sentence reprinted twice; see
[the currency of the flood map](../corpus/measure/allen-county-flood-map-currency-2026.yml).

**One more thing this study is a federal witness to.** Table 27, the community map history, carries
"Fort Shawnee, Village of" with an initial FIRM effective 15 August 1984 and no revision date at all,
under the footnote "This community was dissolved into Allen County, Unincorporated Areas."
[verified] — the 2024 report. That is a fourth kind of record for a dissolution this corpus closed
from a ballot return, and it is the only one that also notes what became of the village's map:
nothing. Its 1984 study still draws three quarters of a mile of the effective FIRM. See
[what happened to the Village of Fort Shawnee](../corpus/question/what-happened-to-the-village-of-fort-shawnee.yml).

**And it names why the Ottawa floods where it does.** "The area of the channel passing the stone
quarry has been rerouted, and the capacity is inadequate for high flows. A sharp turn in the channel
at Schoonover Park is followed by a constricting arch bridge under the Pennsylvania Railroad. This
combination of physical constriction often causes floodwater to back up, increasing flood heights
upstream of the park."
