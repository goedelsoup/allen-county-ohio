---
name: OpenFEMA Assistance Programs
description: >-
  The five OpenFEMA datasets that put dollars beside a disaster declaration: funded public
  assistance projects and the applicants who received them, hazard mitigation projects, and
  housing assistance to owners and to renters. They answer what a declaration cost, who was
  paid, and how much of it reached households, at the grain of a project, an applicant or a
  ZIP code.
type: dataset
obtained: true
retrieved: 2026-09-01
ttl_days: 180
location:
  - kind: url
    value: "https://www.fema.gov/api/open/v2/PublicAssistanceFundedProjectsDetails?$filter=stateAbbreviation eq 'OH' and county eq 'Allen County'"
    description: >-
      Ninety-five project rows, one per project worksheet, with the damage category, the project
      amount, the federal share obligated and the obligation dates. The county filter must read
      "Allen County" and not "Allen" — this dataset spells the county out where the declarations
      dataset does not, and the shorter string returns zero rows silently.
  - kind: url
    value: "https://www.fema.gov/api/open/v1/PublicAssistanceApplicants?$filter=state eq 'Ohio' and applicantId eq '003-99003-00'"
    description: >-
      The applicant register, which turns an `applicantId` into a name and an address. It has no
      county field and its key is not unique: `003-99003-00` returns more than a hundred and eighty
      different county governments across the country, of which the Ohio one is Allen County. The
      state filter is not optional.
  - kind: url
    value: "https://www.fema.gov/api/open/v4/HazardMitigationAssistanceProjects?$filter=state eq 'Ohio' and county eq 'Allen'"
    description: >-
      Three project rows across three programmes — HMGP, FMA — with the project amount, the federal
      share, the cost-share percentage, the benefit-cost ratio and the number of properties. Here
      the county field reads "Allen" without the word County, the opposite of the previous endpoint.
  - kind: url
    value: "https://www.fema.gov/api/open/v2/HousingAssistanceOwners?$filter=state eq 'OH' and county eq 'Allen (County)'"
    description: >-
      Household assistance to owner-occupiers, aggregated to a ZIP code within a disaster: valid
      registrations, inspections, inspected damage, approvals and the money split into repair,
      rental and other needs. A third spelling of the county — "Allen (County)". A companion
      `HousingAssistanceRenters` endpoint carries the same shape for renters, with damage bands in
      place of a dollar figure.
used-by:
  - ../corpus/event/allen-county-declared-for-covid-19-2020.yml
  - ../corpus/event/the-winter-storms-of-2004-2005.yml
  - ../corpus/jurisdiction/city-of-delphos.yml
  - ../corpus/measure/allen-county-disaster-aid-applicants-2005-2025.yml
  - ../corpus/measure/allen-county-disaster-assistance-2005-2025.yml
  - ../corpus/measure/allen-county-hazard-mitigation-2003-2026.yml
  - ../corpus/measure/allen-county-household-disaster-aid-2007-2020.yml
  - ../corpus/organization/lima-memorial-health-system.yml
  - ../corpus/question/what-happened-to-the-village-of-fort-shawnee.yml
---

**What a row is, in each of them.** A *project worksheet* — one item of work an applicant asked to
be paid for, so an applicant appears many times. An *applicant* — a body registered with the state
emergency management agency, which is a government, a school district, a university, a hospital or
a non-profit, never a household. A *mitigation project* — work to reduce a future loss, funded from
an allocation a declaration generates rather than from the damage it caused. And a *ZIP code within
a disaster* — the housing files are aggregates and no row is a household.

**Its county field means different things in different files, and never means the ground.** In the
public assistance file it is the applicant's registered county: Bon Secours Mercy Health, Inc.,
which owns a hospital in Lima, is applicant `061-UWDML-00` and all nine of its projects are counted
in Hamilton County. In the housing files it is the county attached to the registration, and Allen
County's rows under the COVID declaration name Pittsburgh, Richmond Heights and Sherwood among
their cities, because nothing was inspected and there was no property to locate. See
[a grant file counts offices, not ground](../decisions/a-grant-file-counts-offices-not-ground.yml).

**Its money fields are three different things.** `projectAmount` is what the work was scoped at,
`federalShareObligated` is what the federal government committed, and `totalObligated` includes
sums the corpus has not disentangled. The cost share is ordinarily 75 per cent and is not always:
one Allen County project of $23,122 carries a federal share of $1,740.

**The public assistance file begins in the late 1990s.** Allen County's earliest obligation is
20 April 2005 and its ten declarations reach back to 1965; the absence of a 1965 or 1978 row is a
limit of the file and not a fact about the money.

**What else is in it, unread.** `PublicAssistanceFundedProjectsSummaries`,
`PublicAssistanceGrantAwardActivities` and `PublicAssistanceApplicantsProgramDeliveries`;
`HazardMitigationAssistanceMitigatedProperties`, which names the structures a buyout removed;
`HazardMitigationGrantProgramDisasterSummaries`; and forty other OpenFEMA datasets including
firefighter grants and emergency management performance grants.
