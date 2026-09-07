---
name: Ohio EPA, Total Maximum Daily Loads for the Ottawa River (Lima Area) Watershed
description: >-
  The state's 2013 water-quality study of the Ottawa basin. It is read here for two things no
  other source in this catalog says: that the entire tributary network of Hog Creek is
  channelized and legally must stay that way, and which of Lima's low-head dams the state asked
  to have removed.
type: document
obtained: true
retrieved: 2026-09-07
ttl_days: 3650
location:
  - kind: url
    value: https://dam.assets.ohio.gov/image/upload/epa.ohio.gov/Portals/35/tmdl/OttawaLima_Report_Final.pdf
    description: >-
      Final Report, Division of Surface Water, 6 November 2013. 4.5 MB, text layer present,
      about 5,400 lines under `pdftotext -layout`. The field survey it reports is 2010.
used-by:
  - ../corpus/measure/allen-county-dams-2026.yml
  - ../corpus/natural-feature/hog-creek.yml
---

## The passage the corpus took

Section 3.2, on the Upper Ottawa River subwatershed, verbatim:

> The entire drainage network (all tributaries) of Hog Creek have been subjected to extensive
> channelization and other forms of hydromodification, with many streams appearing wholly
> artificial, being cut into the landscape through human activity to drain the associated
> marshlands. These modifications were not limited to tributaries, as Hog Creek itself is
> modified to varying degrees up to the Allen/Hardin county line. Furthermore, the modified areas
> described above are presently petitioned ditches, and as provided by Ohio law, are maintained
> for agricultural drainage. Thus, these and other waters so classified will and must serve as
> outlets and drainage conveyances well into the foreseeable future.

That last sentence is a state agency saying that the county's maintenance obligation is a standing
constraint on what the river can be asked to become. It is the reason this corpus treats a
petition number as part of a watercourse's description and not as an administrative footnote.

The same section says the marsh was there first: *"This area was once a vast wetland complex known
regionally as the Hog Creek marsh."* And of the headwaters: *"agricultural production is only
possible because of the network of artificially drained fields."*

## The dams

> Through urban Lima, a series of five dam pools with five major CSO discharges are contained
> within a three mile reach.

The report recommends removals in phases. Phase 2: *"the Fetter Road dam and the Erie Railroad
(RR) dam near the Lima WWTP."* Phase 3: *"the Baxter Street dam is recommended to be removed."*
Of the Allentown dam, the most downstream, it says only that it *"is a good candidate for
removal"* — and records that at a dam forum on 8 December 2010 *"City officials made a clear
argument for retaining certain dams, and agreed that hydrology study of the Allentown dam was
advisable."* Allentown was removed in August 2020. The other three were not.

## What it is not

It is a load allocation, not a flood study. Nothing in it computes a discharge for a recurrence
interval, and its designations — MWH-C, modified warmwater habitat due to channelization — are
statements about what aquatic life the state expects a channel to support, not about how much
water it carries. For the second question see
[the flood insurance study](fema-flood-insurance-study.md).
