---
name: Census of Agriculture, historical volumes (1954 and 1987, Ohio)
description: >-
  The printed county tables of the United States Census of Agriculture, scanned. Two Ohio volumes
  reach into the ninety-two years this corpus could not see between 1910 and 2002, and each prints
  the census before it beside its own, so two volumes give four census years at county grain.
type: publication
obtained: true
retrieved: 2026-09-01
ttl_days: 3650
location:
  - kind: url
    value: https://archive.org/download/unitedstatescens13unit/unitedstatescens13unit_bw.pdf
    description: >-
      1954 Census of Agriculture, Volume 1 Part 3 — Ohio, Counties and State Economic Areas.
      312 pages. County Table 1 (book p. 44) gives farms, acreage and value for 1954 and 1950;
      County Table 7 (p. 88) livestock; County Table 9 (p. 112) crops. The PDF page is the book page
      plus 30.
  - kind: url
    value: https://archive.org/download/1987censusofagri35unit/1987censusofagri35unit_bw.pdf
    description: >-
      1987 Census of Agriculture, Volume 1 Part 35 — Ohio. Table 5 (book p. 202, PDF p. 216) gives
      farms, land in farms and land use for 1987 and 1982. Allen is the second county column on the
      first page of each table, after the state and Adams.
used-by:
  - ../corpus/measure/allen-county-farms-1949-1987.yml
  - ../corpus/question/when-the-farmland-went.yml
---

**These are page images, and the text layers are not usable.** The 1987 volume's PDF text layer
carries the row stubs and the column headings and drops the numeric grid entirely; the djvu text of
both volumes carries the numbers as an unaligned stream, one token to a line, with thousands
separators broken across lines. A parser fed either would produce numbers with the right shape and
the wrong county. Every figure this corpus takes from these volumes was read off a rendered page
image at 400 dpi. [verified] — both routes exercised, 1 September 2026.

**The scans mistype two digits consistently and arithmetic is the check.** In the 1954 volume a 6
renders as `b` and a 4 as `-`, so "2,5b0" is 2,560 and "262,-00" is 262,400. Every figure taken from
it was verified against another figure in the same column: 227,944 acres over 2,560 farms is 89.0,
which is the average the table prints; 215,018 over 2,097 is 102.5, which is what it prints;
215,018 of 262,400 acres is 81.9 per cent, which is what it prints. A misread digit fails that test.
[verified] — the checks run on every quoted figure.

**Each volume is its own second witness for the census before it.** County Table 1 of the 1954
volume prints 1954 and 1950 in adjacent columns; Table 5 of the 1987 volume prints 1987 and 1982.
That is what makes a pair of figures comparable in a way that two separately published censuses are
not — one publisher, one table, one set of column headings. [inference]

[open] Whether the farm definition moved between the years each volume pairs. The 1954 volume
carries a section on comparability from 1920 onward and this corpus has not read it. The known
break is 1959, which is outside both pairs, and the state totals move with the county in every
comparison made here — which is consistent with a real change and does not prove one.

**What is not here.** No Ohio volume for 1920, 1925, 1930, 1935, 1940, 1945, 1959, 1964, 1969, 1974
or 1978 was found on this host, so the series this corpus can build has a forty-year hole from 1910
to 1950 and a twenty-eight-year hole from 1954 to 1982. Both are now bounded at both ends, which
they were not. [verified] — the host's index searched by title and volume, 1 September 2026.
