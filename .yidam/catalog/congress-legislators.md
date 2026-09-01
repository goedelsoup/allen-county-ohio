---
name: congress-legislators (unitedstates project)
description: >-
  Every person who has ever served in the United States Congress, with each term's chamber, state,
  district, party and exact start and end dates — the community-maintained machine-readable form of
  the Biographical Directory of the United States Congress. It is the first source this corpus holds
  that can check a county history's roster against the federal record it purports to summarize.
type: dataset
obtained: true
retrieved: 2026-09-01
ttl_days: 180
location:
  - kind: url
    value: https://unitedstates.github.io/congress-legislators/legislators-historical.json
    description: >-
      12,231 people, 13 MB, no key and no paging. Each `terms[]` entry carries `type` (rep/sen),
      `state`, `district`, `party`, `start` and `end`. A member who changes district mid-career gets
      one term entry per Congress, so the district field is per-term and not per-person — which is
      the whole reason this file can check a nineteenth-century roster at all.
  - kind: url
    value: https://bioguide.congress.gov/search/bio/<bioguide-id>.json
    description: >-
      The upstream record, and unreachable. Every request returns a Cloudflare 403 interstitial,
      including for ids this corpus holds (N000093, L000039, Y000020). The `id.bioguide` field in the
      file above is the key that would open it if the host answered.
used-by:
  - ../corpus/measure/allen-county-in-congress-1831-1933.yml
  - ../corpus/person/calvin-s-brice.yml
  - ../corpus/person/samuel-s-yoder.yml
  - ../corpus/person/benjamin-f-welty.yml
  - ../corpus/person/mathias-h-nichols.yml
  - ../corpus/person/charles-n-lamison.yml
---

**What it is authority for, and what it is not.** It is authority for who sat, for which state, in
which numbered district, in which Congress, in which party. It is not authority for where a member
lived, and it carries no county at all. So it can say that Ohio's Fourth District returned Matthias
H. Nichols for three Congresses; it cannot say that Allen County was in the Fourth District. Every
claim in this corpus that joins a district to this county rests on the county histories, and this
file is the check on the other half of the sentence. [verified] — the schema, read in full.

**Districts are per term, and that is the finding.** Ohio redrew its congressional map repeatedly in
the 1880s and 1890s, and the file records the consequence rather than the cause: Benjamin Le Fevre
sits for the Fifth, then the Fourth, then the Fifth again in six years; George E. Seney for the
Fifth, the Seventh, and the Fifth; Fernando C. Layton for the Fifth and then the Fourth. A roster
organized by person — which is what a county history writes — has to break a man's service in two to
show it, and the 1906 Allen County history does that for Le Fevre and does not do it for Layton.
[verified] — the file, queried for Ohio districts 3, 4 and 5 between 1831 and 1907; see
[the county in Congress](../corpus/measure/allen-county-in-congress-1831-1933.yml).

**The party field is not decorative.** Nichols holds the same seat for three Congresses as a
Democrat, an "Ind. Republican-Democrat" and a Republican. The file gives party per term for the same
reason it gives district per term, and both are invisible in any list that names a man once.
[verified] — his three term records.

**Birth dates are patchy and death dates are absent.** Of the six members this county claims, the
file gives a birthday for Nichols, Yoder, Layton, Welty, Cable and Brice and none for Lamison, and
it carries no `deathday` field for anyone. The corpus's open questions about when these men died are
not closed by this source and cannot be. [verified] — the six records, read in full.

**Provenance.** The unitedstates project maintains the file from the Biographical Directory and from
subsequent corrections, in the open. It is a transcription of a federal record rather than the
federal record, and it is used here as the second witness against a county history — not as the
first witness against anything. [inference]
