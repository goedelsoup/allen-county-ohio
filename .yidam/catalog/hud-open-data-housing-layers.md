---
name: HUD open data housing layers (ArcGIS feature services)
description: >-
  HUD's live map layers for public housing authorities, developments, buildings, assisted
  multifamily properties, vouchers by tract and fair market rents — keyless, queryable, richer than
  the annual files for anything about condition or structure, and carrying no date anywhere.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 180
location:
  - kind: url
    value: https://services.arcgis.com/VTyQ9soqVukalItT/arcgis/rest/services/Public_Housing_Authorities/FeatureServer/0/query
    description: >-
      One row per housing authority office, with programme sizes, occupancy, operating and capital
      fund amounts and HUD's own physical-condition designation. `where=STATE2KX='39' AND
      CURCNTY='003'`, `outFields=*`, `f=json`.
  - kind: url
    value: https://services.arcgis.com/VTyQ9soqVukalItT/arcgis/rest/services/Public_Housing_Buildings/FeatureServer/0/query
    description: >-
      One row per public housing building, with a building type code, a status code and a unit
      count. 194 rows for Allen County — the only source here that shows what shape the county's
      public housing is in.
  - kind: url
    value: https://services.arcgis.com/VTyQ9soqVukalItT/arcgis/rest/services/MULTIFAMILY_PROPERTIES_ASSISTED/FeatureServer/0/query
    description: >-
      One row per subsidized multifamily property, 21 in Allen County, with contract programme,
      contract expiry, assisted and total unit counts, and the property's last REAC physical
      inspection score and date.
  - kind: url
    value: https://services.arcgis.com/VTyQ9soqVukalItT/arcgis/rest/services/Fair_Market_Rents/FeatureServer/0/query
    description: >-
      Fair market rent by bedroom count for every FMR area. `METRO30620M30620`, the Lima MSA, is
      Allen County and nothing else.
  - kind: url
    value: https://hudgis-hud.opendata.arcgis.com/api/feed/dcat-us/1.1.json
    description: >-
      The catalogue of all 112 HUD layers, which is how the service URLs above were found. Also
      lists Housing Choice Vouchers by Tract and Public Housing Developments.
used-by:
  - ../corpus/measure/allen-county-subsidized-housing-2005-2025.yml
  - ../corpus/organization/allen-metropolitan-housing-authority.yml
---

**Not one of these layers says what date it describes.** There is no year in any row, none in the
layer metadata, none in the service description and none in the DCAT catalogue entry — only a
`modified` timestamp for the service, which dates the publication and not the data. [verified] —
the five services' `?f=json` responses and the catalogue record. The authority and development
layers do carry a per-row `LAST_UPDT_DTTM`, 6 July 2026 for Allen County's, and that is a record's
edit stamp rather than an as-of.
[A file has more than one date](../decisions/a-file-has-more-than-one-date.yml) says an edition year
is a ceiling and not a date; here there is not even an edition, so nothing from these layers is
published as a measure. They are read for **structure** — what shape a thing is, how many of it
there are, what type code it carries — and every figure taken from them is dated in the text by the
day it was retrieved.

**The county field is three characters and a five-digit FIPS returns nothing.** `CURCNTY` is
`'003'`, not `'39003'`; the state lives in `STATE2KX`. A query written as `CURCNTY='39003'` returns
an empty feature array with HTTP 200 and no error, which reads exactly like a county with no public
housing authority. [verified] — it did, on the first four queries run here.

**The place fields are Census 2000 geography and will mislead a reader who trusts them.** `PLACE2KX`
and `PLACE_NM2KX` carry the 2000 vintage: two of Allen County's public housing buildings are filed
under `Fort Shawnee village`, a municipal corporation this corpus reads as having ended between the
2012 and 2013 tax years. [verified] — the buildings layer. It is **not** a witness to anything about
that question; it is a twenty-six-year-old place code travelling in a live file. The current
geography is in `CURCOSUB_NM` and `CURCNTY_NM`, which put the same two buildings in Shawnee Township.

**Its authorized counts match the dated file exactly and its occupied counts do not.** The authority
layer gives Allen Metropolitan Housing Authority 1,301 total units and 1,058 Section 8 units, which
are the 31 December 2025 figures to the unit. The same layer gives 1,253 occupied where that file
gives 1,104, and 1,019 Section 8 occupied where that file gives 889. [verified] — the authority
layer against
[the housing authority file](hud-picture-of-subsidized-households.md), 2025 edition, participant
OH044. So the layer is not simply older: some of its columns are current and some are not, and
without a date there is no way to say which vintage any one of them belongs to.

**What it carries that the annual files do not.** HUD's PHAS physical-condition designation for the
authority; per-property REAC inspection scores and inspection dates; operating and capital fund
amounts and their prior-year values; and, in the buildings layer, a type and status code per
structure. Nothing in the dated series describes condition at all.

**The inspection score is a string and one of Allen County's is `97  b`.** `REAC_LAST_INSPECTION_SCORE`
is typed as text and carries a letter suffix where the inspection found exigent health-and-safety
items; nineteen of the county's twenty-one properties have a plain number and one has a number with
a trailing `b`. [verified] — the multifamily layer. A numeric cast over the column drops or breaks
on that row. The two properties with no score at all are the two newest, which have never been
inspected.

**The scores are dated per row, and the dates span four years.** Allen County's twenty-one
properties were last inspected between 10 March 2022 and 31 March 2026, so no single date describes
the set and the score column is not a snapshot of anything. [verified] — the same layer. This is the
one place in these layers where a figure travels with its own date, which is what
[a file has more than one date](../decisions/a-file-has-more-than-one-date.yml) asks for and the
reason the scores can be published here while the fund amounts beside them cannot.

**Names and addresses were retrieved and are not recorded here.** The authority and multifamily
layers carry executive-director and managing-agent names, direct phone numbers, fax numbers and
e-mail addresses; the buildings layer carries 194 street addresses, each one a place where somebody
lives. This corpus holds the counts, the building types and the civil subdivision, and nothing that
identifies a household — the same line drawn for
[the parcel files](../decisions/auditor-parcels-access-terms.yml) and for hospital ownership under
[what a tract page may be quoted for](../decisions/what-a-tract-page-may-be-quoted-for.yml).
Property names, authority names and public office addresses are institutions and are recorded.

**The voucher-by-tract layer is suppressed below a threshold it does not state.** Seventeen of Allen
County's 35 tracts carry a voucher count and eighteen are null; the seventeen published sum to 930
against 1,005 voucher units in the county. [verified] — the tract layer against the 2025 county
file. The nulls are small counts withheld, not tracts without vouchers, and the layer says so
nowhere.
