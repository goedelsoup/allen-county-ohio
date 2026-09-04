---
name: American Community Survey, 2005–2009 five-year summary file
description: >-
  The first five-year American Community Survey ever published, in the sequence-based format the
  Bureau used before 2021. It is the only file this corpus holds that describes the county's people
  from a window that does not overlap the current one, which is what makes a difference between the
  two a change rather than an arithmetic artefact.
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 3650
location:
  - kind: url
    value: https://www2.census.gov/programs-surveys/acs/summary_file/2009/data/5_year_seq_by_state/Ohio/All_Geographies_Not_Tracts_Block_Groups/20095oh0040000.zip
    description: >-
      Sequence 40 for Ohio, 5.1 MB zipped, holding an estimate file and a margin file with identical
      layouts. Educational attainment — B15002, thirty-five cells beginning at position 90 — is in
      this sequence. Every other table lives in one of the other 116 sequences under the same path.
  - kind: url
    value: https://www2.census.gov/programs-surveys/acs/summary_file/2009/data/5_year_seq_by_state/Ohio/All_Geographies_Not_Tracts_Block_Groups/g20095oh.txt
    description: >-
      The geography file, 7.7 MB, fixed-width, which is the only place a record number is joined to
      a place. Nothing in the data files names a geography.
  - kind: url
    value: https://www2.census.gov/programs-surveys/acs/summary_file/2009/documentation/5_year/user_tools/Sequence_Number_and_Table_Number_Lookup.txt
    description: >-
      The lookup that says which sequence a table is in, where in that sequence it starts, and how
      many cells it occupies. Without it the data files are unreadable.
used-by:
  - ../corpus/measure/allen-county-educational-attainment-2009-2023.yml
---

**Three files are needed to read one number.** The data file carries no geography and no variable
names; the geography file carries no data; the sequence lookup carries neither and is the only
thing that says where a table begins. A reader that has two of the three has nothing. [verified] —
the retrieval here. The table-based file this corpus reads for 2023 folds all three into one, which
is the whole of the difference between the formats.

**The geography file is fixed-width and the join key is seven characters at position 14.** A record
begins `ACSSF OH050` for a county and `ACSSF OH060` for a county subdivision; the logical record
number runs from column 14 to column 20 and the geographic identifier sits at column 179 in the
short form the Bureau used then — `05000US39003`, not the `0500000US39003` of the modern file.
[verified] — the same file.

**Its windows do not overlap the current one, and that is why it was taken.** The 2005–2009 file
and the 2019–2023 file share no year at all, so a difference between them is a difference in the
county and not in the four-fifths of a sample two adjacent vintages hold in common. Every
intervening vintage overlaps one or both. [verified] — the vintages themselves; see
[an overlap names what changed](../decisions/an-overlap-names-what-changed.yml).

**The national file is somewhere else.** The state directory holds Ohio and everything inside it and
stops there; the United States row is published under a separate path this corpus did not take. Every
2009 comparison here is therefore against Ohio, and the national comparisons in this corpus are all
2023. [verified] — the directory listing.

**B15002 has the same thirty-five lines in both formats.** No schooling, nursery to fourth grade,
fifth and sixth, seventh and eighth, ninth, tenth, eleventh, twelfth without a diploma, high school
graduate, some college under a year, some college over a year, associate's, bachelor's, master's,
professional, doctorate — by sex, then a total. The 2023 file's B15003 splits the early grades
further and is not comparable line for line; B15002 is, and is what this corpus used. [verified] —
the two shells.
