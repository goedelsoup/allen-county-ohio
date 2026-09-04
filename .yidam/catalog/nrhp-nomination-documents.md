---
name: National Register nomination documents (NPGallery)
description: >-
  The papers behind the listings. The corpus has held the National Register as a map service — a
  name, a date, a reference number and an arbitrary point — and never the nomination itself, which
  is where a listed structure is described in words and given dimensions. Twenty-eight of
  Allen County's twenty-nine listings have not been scanned; the one that has is the Landmark.
type: document
obtained: true
retrieved: 2026-09-04
ttl_days: 365
location:
  - kind: url
    value: https://npgallery.nps.gov/NRHP/GetAsset/NRHP/66000603_text
    description: >-
      The nomination for the Miami and Erie Canal, Deep Cut — 5 pages, 434,279 bytes, a Form 10-300
      (Rev. 6-72) whose description section is typed and reads cleanly through `pdftotext -layout`.
      The pattern is the reference number followed by `_text`; `_photos` returns the photographic
      documentation as a separate PDF, 1.87 MB for this listing.
  - kind: url
    value: https://catalog.archives.gov/id/71986439
    description: >-
      The same nomination's record at the National Archives, named by the `NARA_URL` field of the
      map service. Not the route used here — the archives' current API wants a key — but it is what
      the spatial dataset points at, and it is how a reader checks that the NPS asset and the
      archived file are the same document.
used-by:
  - ../corpus/measure/miami-and-erie-canal-deep-cut-2026.yml
  - ../corpus/measure/allen-county-national-register.yml
  - ../corpus/site/miami-and-erie-canal.yml
---

**A record that has not been scanned answers 200.** Every reference number returns a PDF. Where the
nomination has not been digitized the PDF is one page, 22,151 bytes, and its entire text is
*"The PDF file for this National Register record has not yet been digitized."* [verified] — the twenty-nine retrievals here. A client that checks the status code and
counts the bytes it received has twenty-nine documents; a client that hashes them has one. The
placeholder is byte-identical across listings, which is what makes the test cheap.

**One of Allen County's twenty-nine.** `66000603`, the Deep Cut, is the only listing in this county
whose nomination is online, and it is also the county's only National Historic Landmark. [verified]
— the same requests, hashed.

That the one scanned document is the one Landmark is a coincidence of two facts about this county
and not evidence of a national rule, and this corpus has not tested the rule anywhere else.
[inference] See [the county's list](../corpus/measure/allen-county-national-register.yml).

**It is worth having because it carries dimensions.** The map service gives a point, a date and a
resource type. The nomination gives a sentence: *"As shallow as five feet, the Deep Cut section of
the Canal ranges at places to 52 feet in depth"*, and *"The Deep Cut extends over a mile."*
[verified] — the document, its description section.

Those are the first stated dimensions this corpus has held for any listed structure in this county,
and they are the reason a [re-measurement](../corpus/measure/miami-and-erie-canal-deep-cut-2026.yml)
was possible at all. A figure can be checked; a coordinate cannot. [inference]

**What it is not.** It is a nomination, not a survey. This one was signed in 1966 and revised on a
1972 form, it names no method, and its author is not given in the digitized pages. Where it is
quoted here it is quoted as a claim made by the National Park Service in 1966 and not as a
measurement; see [a depth needs both of its ends named](../decisions/a-depth-needs-both-of-its-ends-named.yml).
