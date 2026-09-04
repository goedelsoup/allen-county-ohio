---
name: FEMA USA Structures
description: >-
  A polygon for every building in the United States larger than about 450 square feet, machine-drawn
  from aerial photography and machine-classified by what it appears to be. It is the first source in
  this corpus that says where the buildings are, and it is a reading of photographs rather than a
  register of construction.
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 365
location:
  - kind: url
    value: https://services2.arcgis.com/FiaPA4ga0iQKduv3/arcgis/rest/services/USA_Structures_View/FeatureServer/0/query
    description: >-
      The feature service, 2,000 records to a page. Allen County is `FIPS='39003'` and comes to
      59,433 rows in thirty pages, taken without geometry and joined on the file's own centroid
      coordinates; footprints for individual buildings were fetched one query at a time where a
      shape was actually needed.
  - kind: url
    value: https://disasters.geoplatform.gov/publicdata/National_Dataset/USA_Structures/
    description: >-
      The bulk download, one file per state. It answered every request with an HTTP 500 and was not
      used; the feature service carries the same data and pages reliably.
used-by:
  - ../corpus/measure/allen-county-buildings-2019.yml
  - ../corpus/measure/allen-county-buildings-in-the-floodplain-2026.yml
---

**Every row in this county was produced on three days at the end of 2019 from photographs taken on
five earlier days.** 58,519 of the 59,433 carry a production date of 26 December 2019, 880 of 6
January 2020 and 34 of 2 January. The imagery behind them is dated 29 March 2017 for 45,890, 2
December 2018 for 9,532, 7 April 2017 for 2,903, 25 August 2013 for 880 and 27 November 2017 for
228. [verified] — the county's own rows, `PROD_DATE` and `IMAGE_DATE`. Three quarters of Allen
County's buildings are a single flight; see
[a photograph is not a register](../decisions/a-photograph-is-not-a-register.yml).

**Its source and its method are one value each, and they are the same for every building here.**
`SOURCE` is ORNL — Oak Ridge National Laboratory — for all 59,433, and `VAL_METHOD` is *Automated*
for all 59,433. No building in this county was checked by a person. [verified] — the same rows.

**Nine occupancy classes and thirty-one primary occupancies, all of them guesses.** The classes are
Residential, Agriculture, Commercial, Industrial, Government, Education, Assembly, Utility and Misc,
and Unclassified; under them sit Single Family Dwelling, Manufactured Home, Institutional Dormitory,
Religious, Pre-K – 12 Schools, Metals/Minerals Processing and twenty-five more. The classification is
a model's output about a roof, not a record of use, and 736 buildings here defeated it entirely.

**Six of its columns are empty in this county and one of them is the interesting one.** `HEIGHT`,
`H_ADJ_ELEV` and `L_ADJ_ELEV` — the highest and lowest adjacent grade, which is what a flood
analysis wants — are null for every Allen County row, as are `OUTBLDG`, `SEC_OCC` and `B_CODE`.
So are `POP_MEDIAN`, `POP_CI95_LOWER` and `POP_CI95_UPPER`, the modelled population the schema
provides for. [verified] — the same rows. A national file publishes the columns it can fill
somewhere, and a county reading it gets the intersection.

**What was not taken.** `PROP_ADDR` is in the schema and was not requested. The corpus has no use
for a list of the county's dwellings by street number, and the rule it set for the Auditor's parcel
files applies to a federal file just as well; see
[what crosses from the recorder](../decisions/what-crosses-from-the-recorder.yml) and
[the parcel access terms](../decisions/auditor-parcels-access-terms.yml).

**The floor of the dataset is a real edge.** The four smallest footprints in this county are
450.08, 450.08, 450.58 and 450.91 square feet, and nothing below 450 exists in the file at all.
[verified] — the same rows, sorted. Sheds, single garages and the small outbuildings of a farm are
under it, so a count from this file is a count of buildings above a threshold and never of every
roof.
