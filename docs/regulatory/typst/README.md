# Typst filing editions

These source trees are the third-draft review editions of the three contemplated
public filings plus a one-page IAC cover statement. The Markdown documents one
directory above remain Draft 1 research memoranda. The rendered Draft 2 PDFs
remain in `output/pdf/`; the shared Typst sources now produce Draft 3.

Each filing and the cover statement has its own `main.typ`, `metadata.typ`, `body.typ`, and
`sources.typ`. They share presentation helpers only through
`shared/template.typ`; no report imports another report's substance.

Build all four Draft 3 PDFs from the repository root:

```sh
./scripts/build-regulatory-pdfs.sh
```

Stable outputs are written to `output/pdf/`. These are review artifacts, not
authorized submissions. Identity placeholders, live-docket verification,
privacy review, and legal review remain mandatory before filing.
