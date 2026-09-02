---
name: Ohio Election Statistics, 1920 (Secretary of State)
description: >-
  The Ohio Secretary of State's own compilation of the returns certified to him after the general
  election of November 2, 1920 — a county-by-county table for every office on the state ballot,
  including president. It is the first official state election return this corpus holds, and it
  puts a figure in the middle of the ninety-year hole between the county's 1884 returns and its
  first machine-readable file.
type: document
obtained: true
retrieved: 2026-09-01
location:
  - kind: url
    value: https://archive.org/download/ohioelectionsta02statgoog/ohioelectionsta02statgoog.pdf
    description: >-
      24 MB, 471 pages, Google's scan of the State Library copy. **No text layer**: `pdftotext`
      returns only Google's front-matter notice, so every figure here was read from a 400-dpi
      render. The presidential table is book page 11 = PDF page 20, headed "TABLE SHOWING VOTES BY
      COUNTIES FOR CANDIDATES FOR PRESIDENT (FIRST ELECTOR) AT THE ELECTION HELD NOVEMBER 2, 1920."
      Governor follows on book page 13, and the volume runs through Lieutenant Governor, Secretary
      of State, Auditor, Treasurer, the congressional districts, the General Assembly, both
      parties' August primaries, and an abstract of the vote on House Bill 620.
  - kind: url
    value: https://archive.org/download/ohioelectionsta02statgoog/ohioelectionsta02statgoog_djvu.txt
    description: >-
      The OCR, 326 KB. Useful for finding a table and useless for reading one: the tables are
      column-major and the OCR emits the county names in one run and the figures in another, so a
      row cannot be reassembled from it. It is how the presidential table was located and not how
      it was read.
used-by:
  - ../corpus/measure/allen-county-presidential-vote-1920.yml
---

**Why it matters here beyond its figures.** This corpus's only election source until now has been
[OpenElections](openelections-ohio.md), and that entry says plainly what it is: volunteers
transcribing official PDFs, second-hand in a corpus whose other sources publish their own data,
taken because the Ohio Secretary of State's present-day site
[blocks automated clients](allen-county-official-site.md). This volume **is** the Secretary of
State, printing his own certified abstract. The office that refuses the request in 2026 answered it
in 1921, in a book.

**The presidential table's own control.** It prints five vote columns — Cox (Dem.), Harding (Rep.),
Debs (Soc.), Macauley (S.T.), Scattering — and then two plurality columns, Democratic and
Republican. The plurality is the difference of the two leading columns, so every county row checks
itself: Allen's printed Republican plurality of 2,320 is exactly 13,978 less 11,658. A misread digit
in either figure breaks the identity. [verified]

**A note on the two 1920 volumes.** archive.org holds two items both titled *Ohio Election
Statistics* and both dated 1920 — `ohioelectionsta02statgoog` and `ohioelectionsta03statgoog`. Only
the first was read. The rest of the series is `ohioelectionsta00statgoog` (1895),
`ohioelectionsta04statgoog` (1901) and `ohioelectionsta01statgoog` (1905), and the 1901 volume
carries a running statewide table of the vote at every presidential and state election back to
1803 — **state totals only**, which is why it does not close this county's gap.

**What it does not carry.** No precincts and no townships: the county is one row. For anything
below the county line in 1920 this corpus has nothing, and the last sub-county returns it holds
before 2016 are the twenty township and ward figures the
[1885 county history](leeson-allen-county-1885.md) prints for 1884.
