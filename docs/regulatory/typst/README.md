# Typst filing editions

These source trees are the current review editions of the contemplated
public filings (two joint comments, the 24/7/perpetuals comment, and the
IAC written statement) plus a one-page IAC cover statement. The Markdown
documents one directory above remain Draft 1 research memoranda. Earlier
draft PDFs remain in `output/pdf/`; each tree's `metadata.typ` names the
draft its sources currently produce.

Each filing and the cover statement has its own `main.typ`, `metadata.typ`, `body.typ`, and
`sources.typ`. They share presentation helpers only through
`shared/template.typ`; no report imports another report's substance.

Build all five current PDFs from the repository root:

```sh
./scripts/build-regulatory-pdfs.sh
```

Stable outputs are written to `output/pdf/`. These are review artifacts, not
authorized submissions. Identity placeholders, live-docket verification,
privacy review, and legal review remain mandatory before filing.

The August 19 Draft 9 editions of the two joint comments and the IAC statement
and cover distinguish scoped formal/host/local-SBF evidence from the proposed
authenticated-source-to-resolution join. They make no production-readiness,
deployment, whole-protocol-verification, provider-authentication, or legal-
classification claim. The perpetuals comment remains Draft 2.
