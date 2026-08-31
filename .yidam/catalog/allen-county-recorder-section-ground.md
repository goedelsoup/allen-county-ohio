---
name: Allen County Recorder — Section Ground book list
description: >-
  The Recorder's own finding aid to its tract index: thirty Section Ground volumes and the
  townships, ranges and section runs each one abstracts. It is not title data and contains no
  person. It is the map from a piece of ground to the book that holds that ground's history, and
  it is what makes a chain of title retrievable by asking for a section rather than a name.
type: document
obtained: true
retrieved: 2026-08-30
location:
  - kind: url
    value: https://recorderexternal.allencountyohio.com/PaxWorld/views/Archives
    description: >-
      The archive browse page, behind the office's login. `AbstractBooks-BookLists-SectionGround.pdf`
      is the printed book list it serves. Supplied to this corpus by the account holder; the
      archive itself is not reached from this repository.
---

**What Section Ground is.** The Recorder's **tract** index — the county's records organized by
*ground* rather than by name. Its rows carry grantor, grantee, book, page, instrument type, date
executed and date recorded as separate columns, an aliquot description, and a mortgages section
with its own cancelled-and-released column. It runs from the federal patent forward.

It is also, unlike the deed books and the grantee indices, **typewritten** — the county re-typed
and certified its abstracts between about 2008 and 2020. That is the difference between a source
a person must read and a source a pipeline can.

None of that is in this corpus. What is here is the **book list**, and only the book list.

## What the list says

Thirty volumes, organized by **civil township** and split into lettered parts by section run:

    26, 26A, 26A-1        Richland          31, 31A, 31A-1, 31A-2   American
    27, 27A               Monroe            32, 32A                 Jackson
    28, 28A, 28A-1,       Bath              33, 33A, 33A-1          Perry
      28A-2, 28B (Sugar Creek)              34, 34A, 34A-1          Shawnee
    29, 29A               Marion            35, 35A                 Auglaize
    30, 30A               Amanda            36, 36A                 Spencer

A volume is one township, not one range. [verified] That corrects a reading taken from a single
volume, which found that book 0026 never changes range and inferred the organizing principle from
it — true of that book and not the rule.

## It nearly closes against the survey, and the residue is the useful part

The runs expand to 416 section-slots against the county's
[404 real sections](allen-county-gis-rest.md):

| | |
|---|---|
| in a book and in the survey grid | 402 |
| **in two books** | 1 — T4S R5E §8, in Amanda's `30A` and Spencer's `36` |
| **in the grid and in no book** | 1 — T4S R5E §18 |
| in a book and not in the grid | 13 — T3S R4E §28–33, T4S R4E §4–9, T2S R8E §32 |

The thirteen sit at the county's western edge, where a survey township crosses into Van Wert. A
Recorder's township volume following the survey rather than the county line would explain all of
them and this corpus has not checked that it does. [open]

`ground::books_for` returns a list and not a single answer for exactly this reason: two of the
county's own section-to-book assignments are not functions, and a lookup promising one answer
would have to invent one.

## What has not been taken, and why

The books themselves. Retrieval of archive pages is free and the office has settled programmatic
access at one page a second, so this is a choice rather than a limit — see
[what crosses from the recorder](../decisions/what-crosses-from-the-recorder.yml). The book list
is a finding aid; the volumes are title records naming private parties, and this corpus takes a
page only against a question it has already written down.

## used-by

- `crates/ground` — `books_for`, and the committed `section-ground-books.json`
- [`ground-at`](../skills/ground-at.md)
- [Allen County survey sections](../corpus/measure/allen-county-survey-sections.yml)
