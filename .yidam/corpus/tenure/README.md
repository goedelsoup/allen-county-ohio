# tenure

A tenure is one person's holding of one office over one interval. It is the node that makes
"who was sheriff in 1893" a question the corpus can answer rather than a sentence somebody
has to have written down.

The class exists because the alternative encodings both fail. Put the office on the person
and a forty-year career becomes an unqueryable paragraph. Put the person on the office and
the office node grows without bound and cannot express two people holding it at once. A
tenure node holds the pair plus its own dates, how it began, and how it ended — and how it
ended is often the most historically interesting field, because resignations, deaths in
office and removals are where the record gets interesting.

**This class was empty at genesis** — see
[`../../decisions/seed-scope.yml`](../../decisions/seed-scope.yml), which records why nothing
was invented to fill it. It now holds the complete line of Allen County sheriffs, 39 tenures
from 1831 to the present, extracted from the office's own roster. Every one of them leaves
`how_began` and `how_ended` absent, because the roster gives year ranges and names and nothing
else.

Class definition: [`../tenure.ont.yml`](../tenure.ont.yml)
