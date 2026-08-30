---
name: CMS Hospital General Information (Care Compare)
description: >-
  The federal roster of hospitals in the Medicare quality-reporting programme — name, address,
  county, type, ownership form, emergency services and overall star rating for every one. It is the
  first source in this corpus that names a private employer operating in Allen County today.
type: dataset
obtained: true
retrieved: 2026-08-30
ttl_days: 180
location:
  - kind: url
    value: https://data.cms.gov/provider-data/api/1/datastore/query/xubh-q36u/0
    description: >-
      Keyless JSON. Filter with `conditions[n][property]=state|countyparish`. Allen County, Ohio is
      `state=OH` and `countyparish=ALLEN`; Ohio's whole slice is 193 rows.
---

**It names four hospitals in Allen County.** Lima Memorial Health System, 1001 Bellefontaine Avenue,
Lima — acute care, voluntary non-profit – other, with an emergency department and an overall rating
of 3. Mercy Health St Rita's Medical Center, 730 West Market Street, Lima — acute care, voluntary
non-profit – private, emergency department, rating 4. Institute for Orthopaedic Surgery, 801 Medical
Drive Suite B, Lima — acute care, proprietary, **no** emergency department, rating not available.
Bluffton Hospital, 139 Garau Street, Bluffton — critical access, voluntary non-profit – private,
emergency department, rating not available. [verified]

**Four is this file's answer and not the county's.** The Ohio slice runs to 193 hospitals in six
types — acute care, critical access, psychiatric, children's, and two federal categories — and no
long-term care category at all. Allen County's fifth Medicare hospital,
[Kindred Hospital Lima](cms-hospital-cost-reports.md), is a long-term acute care hospital and is
absent from all 193 rows: no CCN 362020, and the only row at its street address is St. Rita's.
[verified] The file is a roster of the hospitals in one quality programme, and it is exact about
that.

This corpus therefore does not treat this file as an enumeration of the county's hospitals, and no
node here cites it for a count. It is cited for what each named hospital *is* — type, ownership
form, whether it takes emergencies — which is the thing it is authoritative about and the thing
the cost reports and the enrollment files do not say.

**Ownership form is the field that earns its keep.** Three of the four are non-profit and one is
proprietary, and the file splits non-profit into "other" and "private" — Lima Memorial in the first,
St. Rita's and Bluffton in the second. That is a coarser statement than
[the enrollment files](cms-provider-enrollment.md) make, and the two agree: the hospital the file
calls non-profit – other is the one the enrollment file shows has no corporate owner at all.
[inference]

**What else is in it, unread.** Sixty-odd columns of quality measurement — mortality, safety,
readmission and patient-experience group counts, and how many of each measure a facility scored
better, worse or no differently than the national rate. This corpus has taken the identity columns
and the rating, and nothing about care.
