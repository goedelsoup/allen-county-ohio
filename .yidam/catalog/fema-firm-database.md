---
name: FEMA FIRM Database, Allen County (NFHL county download)
description: >-
  The same flood map as the National Flood Hazard Layer service, downloaded as the FIRM Database:
  ninety-eight files, twenty-one shapefiles and thirteen standalone tables. The tables are why it
  is a separate source. The map service publishes the geometry and one lookup; the download
  publishes the twelve the service withholds, and those are where FEMA records which study drew
  each line, when the engineering behind it was finished, and which hydraulic model produced it.
type: dataset
obtained: true
retrieved: 2026-09-07
ttl_days: 365
location:
  - kind: url
    value: https://hazards.fema.gov/nfhlv2/output/County/39003C_20240620.zip
    description: >-
      The county database, 20.2 MB, named for the effective date. Unzips to 98 files with no
      containing directory. The shapefiles are NAD83 geographic with a NAVD88 vertical, so lengths
      and areas need projecting before they mean anything.
  - kind: url
    value: https://msc.fema.gov/portal/advanceSearch
    description: >-
      The Map Service Center, which is where the download is found by hand and where the Flood
      Insurance Study report that documents it is served; see
      [the study](fema-flood-insurance-study.md).
used-by:
  - ../corpus/measure/allen-county-flood-map-currency-2026.yml
---

**The thirteen tables are the point, and twelve of them are only here.** The map service exposes
thirty-two feature layers and exactly one table, `Study_Info`. The download carries `L_SOURCE_CIT`
(53 rows), `S_SUBMITTAL_INFO` (18), `L_PAN_REVIS` (49), `L_COMM_INFO` (11), `L_COMM_REVIS` (21),
`L_XS_ELEV` (3,579), `L_SUMMARY_DISCHARGES` (303), `L_XS_STRUCT` (113), `L_MANNINGSN` (24),
`L_MEETINGS` (11), `L_MTG_POC` (19), `L_POL_FHBM` (6) and `STUDY_INFO` (1). [verified] — counted
here.

**Every feature carries a `SOURCE_CIT` code and the service cannot tell you what it means.** A
profile baseline says `39003C_FIS4`; only `L_SOURCE_CIT` says that this is the Flood Insurance
Study for the City of Lima of 15 August 1978. The service serves the code and not the citation, so
a question about a line's provenance is answerable from the download and unanswerable from the API
this catalog already held; see [the map service](fema-nfhl.md).

**`S_SUBMITTAL_INFO` is the table that dates the engineering.** Its columns are `CASE_NO`,
`COMP_DATE`, `TASK_TYP`, `HYDRO_MDL`, `HYDRA_MDL`, `TOPO_SRC`, `CONTRCT_NO` and `METHOD_TYP` — the
FEMA case, the date the work was finished, the models used, and whether the work was new hydrology
and hydraulics or a redelineation of somebody else's. Allen County's eighteen rows all sit under
case `14-05-4448S`, and twelve of them give `COMP_DATE` as 15 October 2018. [verified]

**`METHOD_TYP` is the field that distinguishes new water from a new line.** Eleven of the eighteen
rows read `REDELINEATION` and carry `HYDRA_MDL` values of `HEC-2 4.6.2 (May 1991)`,
`TR-20 Win (Feb 1992)` and `TR-20 Win 1.00 (Jan 2005)` — the programs whose old results were
redrawn — against three rows of `New H&H`. [verified] See
[a redelineation is a new line on an old number](../decisions/a-redelineation-is-a-new-line-on-an-old-number.yml).

**It is not UTF-8 and a strict reader dies on it.** `L_SOURCE_CIT` is cp1252: the string
`Fact Sheet FS 108–00` stores its en dash as byte `0x96`, and pyshp's default decoding raises
rather than substituting. Open every `.dbf` here with `encoding='cp1252'`. [verified] — observed
on the first read.

**The database disagrees with its own report about when the map took effect.** `S_FIRM_PAN.EFF_DATE`
gives 20 June 2024 for thirty-seven panels, and the study report is titled "REVISED: JUNE 20, 2024".
`S_SUBMITTAL_INFO.EFF_DATE`, `L_COMM_REVIS.REVIS_DATE` and `L_COMM_INFO.RECENT_DAT` all give
**22 May 2024**, and four `L_SOURCE_CIT` rows describe their contribution as shown "on FIRM panels
dated 5/22/2024". [verified] — all five tables, read here. Twenty-nine days, three tables against
two, and nothing in the package says which is the date.

**And it names a metadata file that was never written.** `STUDY_INFO.META_NM` points at
`39003C_20240620_metadata.xml`, which is in the package and is fifty-seven bytes long: the words
"NFHL metadata for `<DFIRM_ID>`" and "Update Date: `<UPD_DATE>`", placeholders unfilled.
[verified] — the file, read here.
