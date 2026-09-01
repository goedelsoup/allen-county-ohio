---
name: Supreme Court of Ohio attorney directory
description: >-
  The Office of Attorney Services' public register of every person ever admitted to the practice
  of law in Ohio, searchable by name, by attorney registration number, and by the county of the
  address the attorney has given the Court. It returns the admission date to the day, how the
  person was admitted, and the current registration status. It is the first source in this corpus
  that dates a living officer's professional qualification, and the first that carries a
  hundred-year cumulative roll of a single county's lawyers.
type: dataset
obtained: true
retrieved: 2026-09-01
ttl_days: 180
location:
  - kind: url
    value: https://www.supremecourt.ohio.gov/AttorneySearch/
    description: The search page — an Ember single-page application with no server-rendered content
  - kind: url
    value: https://www.supremecourt.ohio.gov/AttorneySearch/Ajax.ashx
    description: >-
      The endpoint the page posts to. `action=SearchAttorney` with `attyReg`, `firstName`,
      `lastName`, `middleName`, `address`, `city`, `state`, `zip` and `county`, plus an
      `X-CSRF-TOKEN` header copied from the `csrf-token` meta tag of the search page. The token is
      a fixed string served to every visitor and no cookie is set, so the guard is nominal.
      **`myLanguages` must be present and must be a JSON string** — the array of five language
      objects that `action=LoadSearchOptions` returns. Sent as a jQuery-style bracketed array, or
      omitted, the request fails with "Value cannot be null. Parameter name: value" and no
      indication of which parameter. That one encoding is the whole difficulty of this source.
  - kind: url
    value: https://www.supremecourt.ohio.gov/AttorneySearch/scripts/dist/templates.js
    description: >-
      The compiled templates, which carry the status definitions and the address disclaimer. They
      are not reachable from the rendered page without opening a panel, and they are the only
      documentation of what the fields mean.
used-by:
  - ../corpus/measure/allen-county-attorney-register-2026.yml
  - ../corpus/person/anthony-layton-geiger.yml
  - ../corpus/person/destiny-rae-caldwell.yml
  - ../corpus/person/james-d-jordan.yml
  - ../corpus/person/jeffrey-l-reed.yml
  - ../corpus/person/john-r-willamowski.yml
  - ../corpus/person/john-richard-payne.yml
  - ../corpus/person/juergen-a-waldick.yml
  - ../corpus/person/mark-c-miller.yml
  - ../corpus/person/matt-c-staley.yml
  - ../corpus/person/matthew-c-huffman.yml
  - ../corpus/person/susan-manchester.yml
  - ../corpus/person/tammie-k-hursh.yml
  - ../corpus/person/terri-lynn-kohlrieser.yml
  - ../corpus/person/todd-e-kohlrieser.yml
  - ../corpus/person/william-r-zimmerman.yml
  - ../corpus/place/bluffton.yml
  - ../corpus/place/delphos.yml
---

**The county field is a business address, and the source says so.** Its own disclaimer reads: "The
directory lists an attorney's business address. An attorney's residence address is displayed only
if the attorney has not provided a valid business address. See, Gov. Bar R. VI, Sec. 6(B)." The
search result does not say which of the two you are looking at. [verified] That is the reason this
source cannot answer the question seven nodes in this corpus have open — where a district officer
lives — and it is worth more than the answer would have been, because it converts seven "not
looked" markers into one cited refusal.

**It is cumulative and it is never purged.** A county query returns everyone whose address of
record is in that county, admitted at any time. Allen County's earliest is admitted 6 January 1925.
[verified] Nothing in a result marks a person as dead.

**Its status vocabulary, verbatim from the definitions panel** [verified]:

      Active         may practice law in Ohio, assuming all other requirements are met
      Inactive       may not practice, or hold themselves out as authorized to practice
      Retired        must have been at least 65 at the time of the retired registration;
                     "no longer available as a registration status, effective September 1, 2007"
      Not Required   "an attorney who is not required to register with the Office of Attorney
                     Services for a variety of reasons"

**One of those four is not a definition.** "For a variety of reasons ... please contact the Office
of Attorney Services" is the source declining to say what its own second-largest category means.
The corpus records the count and does not name the reasons. [verified] — the same panel.

**`AdmittedBy` distinguishes the routes in.** By Exam, By Motion, UBE Transfer, and — on one Allen
County record — Not Admitted, which is paired with an admission date of `N/A`. [verified]

**The county filter is a substring match and it is not scoped to Ohio.** `county=Alle` returns the
1,000-result cap; `county=Allen` with `state=IN` returns zero, which is how the Allen County result
was confirmed to be the Ohio one rather than Fort Wayne's. [verified] The check was run because
[a federal grant file](openfema-assistance-programs.md) once returned another state's county under
an identifier this corpus assumed was local.

**The cap is 1,000 and it announces itself.** A response carries `TooManyResults`, and the template
explains: "Your search returned a large result set! The first 1000 results are listed below."
Franklin County returns 1,001 rows with the flag set. Allen County returns 334 with it clear, so
that figure is a whole answer and not a truncation. [verified]

**Its county field is self-reported and incomplete, and the county's own geography proves it.**
Of 305 records with a Lima, Ohio address, 298 carry `county=Allen` and seven carry no county at
all — and Lima lies wholly within Allen County. Against that, [Delphos](../corpus/place/delphos.yml)
returns twenty records that divide thirteen Allen and seven Van Wert with none left over, and
[Bluffton](../corpus/place/bluffton.yml) returns eight that divide six Allen and two Hancock with
none left over. The field is complete for the two cities the county line runs through and
incomplete for the one it does not. [verified]

**A name search finds only the name now on file.** The source says the record "displays the
attorney's current name on file in our records", so a person admitted under one name and
registered under another is reachable only under the second. [verified] A null result for a name
this corpus holds is therefore not evidence that the person was never admitted, and no such result
is read that way here.

**What it does not carry in a search result.** No employer, no address, no law school, no
discipline history — those are on the individual record behind a second request keyed to the
attorney number, and this corpus has not fetched them. No date of birth. No indication that a
person has died. No link between a registrant and any office they hold, which is why every join
from this source to a named officer in this corpus is an inference on a name.
