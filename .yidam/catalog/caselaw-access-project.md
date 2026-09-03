---
name: Caselaw Access Project (static files)
description: >-
  Harvard's digitisation of the American case reporters, published as plain static JSON with no key,
  no form and no rate limit. It is where this corpus reads an opinion once
  [CourtListener](courtlistener.md) has found it.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 3650
location:
  - kind: url
    value: https://static.case.law/f-supp/384/cases/1196-01.json
    description: >-
      *Davis v. Watkins*, 384 F. Supp. 1196 — 60 KB of JSON carrying the full opinion text, the
      court, the docket number, the decision date and the authoring judge. The file name is the
      first page of the case in the volume plus an ordinal.
  - kind: url
    value: https://static.case.law/f-supp/384/CasesMetadata.json
    description: >-
      The volume's table of contents, 209 cases and 1.6 MB, which is how a citation is turned into
      a file name. Volumes 461 and 506 were read the same way for the two later opinions in the same
      case.
  - kind: url
    value: https://static.case.law/
    description: >-
      The index of reporters. Federal Supplement is `f-supp`; the whole of the American case
      reporters is here under similar names.
used-by:
  - ../corpus/event/davis-v-watkins.yml
  - ../corpus/measure/lima-state-hospital-in-the-reports-1920-2021.yml
---

**Three opinions in one case, one docket, one judge, six years.** *Davis v. Watkins*,
384 F. Supp. 1196 (9 September 1974); *Davis v. Balson*, 461 F. Supp. 842 (28 September 1978); and
*Davis v. Hubbard*, 506 F. Supp. 915 (16 September 1980). All three are the Northern District of
Ohio on docket C 73-205 before Judge Walinski, and the defendant's name changes because the
superintendent did. [verified] — the three files, read here.

**The first is a remedy and the two after it are findings, and the first says so.** The 1974 order
states in its second paragraph that the court "agrees almost totally with the reasoning" of
*Wyatt v. Stickney* and that "many of these paragraphs have been excerpted verbatim from that
opinion" — an Alabama case about Alabama institutions. [verified] — 384 F. Supp. 1196. The specific
prohibitions it contains are therefore evidence about Alabama's litigation and not about this
county; see [a remedy is not a finding](../decisions/a-remedy-is-not-a-finding.yml).

**Text arrives with the reporter's typographic errors intact.** The 1974 order spells the hospital's
newer building *Aseherman* where the 1978 and 1980 opinions spell it *Ascherman*, and the 1980
opinion carries `7' X 11X` where a fraction was set. These are scanning artefacts of the printed
reporter rather than the court's spelling, and a quotation taken from here carries them. [verified]
— the three files.
