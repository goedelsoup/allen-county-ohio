---
name: Census Bureau public employment and payroll, individual unit files
description: >-
  Every unit of state and local government in the United States, with how many people it employs and
  what it paid them in March, by function. Once every five years it is a complete census; in between
  it is a sample. It is the only source in this corpus that counts the employees of a township.
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 365
location:
  - kind: url
    value: https://www2.census.gov/programs-surveys/apes/datasets/2022/2022%20COG-E%20Individual%20Unit%20Files.zip
    description: >-
      The 2022 Census of Governments employment component, 33.6 MB. `22empid.txt` is one fixed-width
      row of 213 characters per government unit — name, type, county, population or enrollment — and
      `22empst.txt` is one 80-character row per unit and function, carrying full-time and part-time
      employees, their March payroll, and a data flag for each of the four. The two join on the
      first fourteen characters.
  - kind: url
    value: https://www2.census.gov/programs-surveys/apes/datasets/2024/2024_individual_unit_files.zip
    description: >-
      The 2024 annual survey, same layout. Annual years are a sample: it reached two of Allen
      County's forty-five governments, the county at a selection probability of 0.9091 and the City
      of Lima at 1.0000. Files run back to 1992, in this layout from 2017 and in an older one before.
used-by:
  - ../corpus/measure/allen-county-governments-and-their-employees-2022.yml
---

**A census year and a survey year are different objects and the directory does not say so.** 2022 is
the Census of Governments and enumerates every unit; 2024 is the Annual Survey of Public Employment
& Payroll and samples them. Both arrive as `individual_unit_files.zip` under `apes/datasets/`, both
have the same layout, and the only visible difference is how many rows a county has. Allen County
has 45 in 2022 and 2 in 2024. [verified] — both files, filtered on FIPS 39003.

**The county key is in the ID file and not in the data file.** `22empst.txt` carries the Bureau's own
legacy identifier — state code, unit type, county code, unit number — and no FIPS at all; FIPS state
and county sit at positions 110–111 and 112–114 of `22empid.txt`. Ohio is state code 36 in the
Bureau's government scheme and 39 in FIPS, and Allen County is 002 in the first and 003 in the
second. A filter written against the wrong pair finds Ashland. [verified] — the technical
documentation and the two files.

**Payroll is a March figure, standardised to 31 days, and it is not a twelfth of a salary.**
[verified] — the technical documentation, section 1.1. See
[a March payroll is not a salary](../decisions/a-march-payroll-is-not-a-salary.yml).

**Every number carries a flag saying whether anyone reported it.** `R` is reported, `C` and `K` are
analyst corrections to reported data, and `A`, `B`, `D`, `G`, `J`, `P`, `Q` and `X` are kinds of
imputation. Of Allen County's 45 governments in 2022, 28 reported their full-time employment
directly, 8 reported it with an analyst correction, 2 reported a unit total that was pro-rated
across functions, and **7 did not report at all** — their figures are the previous year's, grown by
a rate taken from similar units that did respond. [verified] — the same file, position 32.

**Thirty-three functional categories, and the one a reader wants is usually two.** Police is split
between officers with the power of arrest and everyone else, fire between firefighters and everyone
else, education between instructional and other, and there is an `All other and unallocable` bucket
that in this county holds 199 full-time people. Summing a function across units is safe; comparing
one county's *police* with another's needs both police codes.
