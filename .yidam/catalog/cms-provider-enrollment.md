---
name: CMS hospital provider enrollment, ownership and change of ownership
description: >-
  The three public extracts of Medicare's provider enrollment system for hospitals: who is enrolled
  and under what legal name, who owns them, and which ones changed hands. It is the only source in
  this corpus that separates the name on a building from the body that answers for it, and the only
  one that says where the county's largest private employers are owned from.
type: dataset
obtained: true
retrieved: 2026-08-30
ttl_days: 180
location:
  - kind: url
    value: https://data.cms.gov/sites/default/files/2026-08/bcf5ccda-1482-4c43-8198-de1efa2f83c8/Hospital_Enrollments_2026.07.31.csv
    description: >-
      Hospital Enrollments, as of 31 July 2026. 9,161 rows, 40 columns. Legal organization name,
      doing-business-as name, incorporation date and state, organization type, address, and
      thirteen subgroup flags. Latin-1, not UTF-8.
  - kind: url
    value: https://data.cms.gov/sites/default/files/2026-08/c07d8f27-bd6c-409b-b0f5-958c71923ce9/Hospital_All_Owners_2026.07.31.csv
    description: >-
      Hospital All Owners, as of 31 July 2026. 146,859 rows keyed to the enrollment file by
      ASSOCIATE ID. Each row is one owner — organizational or individual — with role, percentage,
      association date and eighteen type flags.
  - kind: url
    value: https://data.cms.gov/sites/default/files/2026-07/c11c9d53-72a0-4ed2-95b0-4770c77bfd2c/Hospital_CHOW_2026.07.17.csv
    description: >-
      Hospital Change of Ownership, as of 17 July 2026. 772 rows, buyer and seller side by side,
      effective dates from 1 January 2016 to 31 December 2024.
---

**Every hospital in Allen County trades under a name that is not its own.** All five, without
exception —

    doing business as                        legal organization name                incorporated
    Lima Memorial Health System              Lima Memorial Joint Operating Company  12 May 1999, OH
    Mercy Health–St. Rita's Medical Center   Mercy Health-St Ritas Medical           5 May 1970, OH
                                               Center LLC
    Institute for Orthopaedic Surgery        West Central Ohio Group Ltd.           10 Mar 1997, OH
    Bluffton Hospital                        Blanchard Valley Regional Health       6 Jul 1982, OH
                                               Center
    Kindred Hospital Lima                    SCCI Hospitals of America LLC          not stated

[verified] Two of the five are unrecognisable from the sign: the county's orthopaedic hospital
answers to a body named for a region, and its hospital in Bluffton answers to one named for a river
valley in the next county.

**And it says who owns them, which nothing else this corpus holds does.** St. Rita's is wholly owned
by Bon Secours Mercy Health Inc of Cincinnati, from 1 January 2020. Bluffton Hospital is wholly
owned by Blanchard Valley Health System of Findlay, in Hancock County, from 1983. The Institute for
Orthopaedic Surgery is 51 per cent owned by St. Rita's directly and 49 per cent by West Central Ohio
Physicians Group Ltd., which holds its operational and managerial control. Kindred Hospital Lima
has thirteen organizational owners, every one of them in Louisville, Kentucky: one direct owner,
SCCI Health Services LLC, and twelve indirect ones, the most recent three attached on
23 December 2021 and named for Knight rather than Kindred. Lima Memorial has no organizational owner in the file at all. [verified]

**Individual owners were retrieved and are not recorded.** The owners file names officers,
directors and managing employees personally: 22 of them for Lima Memorial, 40 for St. Rita's, 101
for Bluffton, 16 for Kindred, 8 for the Institute. This corpus records the counts and the roles and
not the names.

That is a narrower restraint than the one on
[parcel owner data](../decisions/auditor-parcels-access-terms.yml), and the difference is worth
stating rather than blurring. There the files were never fetched, because the corpus had no question
that needed a private individual's name attached to their home. Here the file was fetched — the
organizational ownership is in the same rows — and the restraint is in what was written down. These
are disclosures about a role in a regulated body, not about a residence, and a later phase with an
actual question about hospital governance could reach them again from the same URL.

**No Allen County hospital changed hands between 2016 and 2024.** The change-of-ownership file
covers exactly that span in 772 transactions, 23 of them in Ohio, and none of them names any of the
five CCNs or their three distinct-part units. [verified] So the ownership above is not a snapshot
of a moving thing: within the nine years this file can see, it did not move.

**Its unit is the enrollment, not the hospital.** Allen County has eight enrollment records: five
hospitals and three further records carrying a letter in the third position of the CCN — 36T009 and
36T066, flagged as rehabilitation units of Lima Memorial and St. Rita's, and 36Z322, a second
Blanchard Valley Regional Health Center record with no trading name and no subgroup flag set. Counting rows would give the county eight hospitals. [verified] The same eight are
returned whether the file is filtered by city name or by Allen County ZIP code, which is the check
that the set is closed rather than the query being lucky.

**Two of them are at the same street address.** Kindred Hospital Lima enrolls at 730 West Market
Street, which is St. Rita's. A long-term acute care hospital operating inside a host hospital is an
ordinary arrangement and the file records it without comment. [inference] It means the county's
five hospitals stand at four addresses, and that the sign on a building is not a count of the bodies
inside it either.

**What else is in it, unread.** The subgroup flags — psychiatric, rehabilitation, swing-bed,
children's, specialty — for every hospital in the country; the REH conversion flag and date, which
marks hospitals that became rural emergency hospitals; and NPI and enrollment identifiers that would
join these rows to the rest of the Medicare provider files.
