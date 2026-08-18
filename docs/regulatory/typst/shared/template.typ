// Shared house style for the regulatory filing package.
//
// Typography: STIX Two Text (variable, 400-700) throughout. It is a Times-family
// text face built for scientific and legal publishing: real small capitals, real
// italics, lining tabular figures, and a full citation glyph set.

// ---------------------------------------------------------------- palette ----

#let ink = rgb("#16202f")
#let muted = rgb("#5b6678")
#let rule = rgb("#d2d9e3")
#let accent = rgb("#1d4e79")
#let pale = rgb("#eef3f9")
#let warning = rgb("#fdf5e0")
#let amber = rgb("#b07515")
#let alarm = rgb("#9c3218")

// ------------------------------------------------------------ typography ----

#let body_font = "STIX Two Text"
#let body_size = 10.5pt
#let body_lead = 0.68em
#let body_space = 1.18em

// A small-caps label with open tracking; used for headings, table headers,
// callout titles, and the running head.
#let sc_label(body, size: 8.5pt, fill: muted, weight: "semibold", tracking: 0.07em) = {
  text(size: size, fill: fill, weight: weight, tracking: tracking, smallcaps(body))
}

// ------------------------------------------------------------- fragments ----

#let note_ref(number) = super(text(fill: accent, weight: "semibold", str(number)))

#let review_note(body) = block(
  width: 100%,
  fill: warning,
  stroke: (left: 2.5pt + amber),
  inset: (left: 12pt, right: 13pt, top: 9.5pt, bottom: 9.5pt),
  radius: (right: 2pt),
  breakable: true,
  above: 13pt,
  below: 13pt,
  {
    set par(leading: 0.7em, spacing: 0.8em, justify: false)
    body
  },
)

#let key_point(title, body) = block(
  width: 100%,
  fill: pale,
  stroke: (left: 2.5pt + accent),
  inset: (left: 13pt, right: 14pt, top: 11pt, bottom: 12pt),
  radius: (right: 2pt),
  breakable: false,
  above: 13pt,
  below: 13pt,
  {
    set par(leading: 0.7em, spacing: 0.95em)
    block(above: 0pt, below: 10pt, sc_label(title, fill: accent, size: 8.8pt))
    body
  },
)

// A short footnote rule, for source notes set as a bottom-of-page block.
#let note_divider = block(above: 13pt, below: 7pt, line(length: 1.8in, stroke: 0.5pt + rule))

#let source_entry(number, title, url, detail) = block(
  width: 100%,
  breakable: true,
  above: 8pt,
  below: 8pt,
  par(hanging-indent: 1.6em, leading: 0.66em, justify: true)[
    #text(weight: "bold", fill: accent)[#number.]#h(0.5em)#link(url)[#title]. #detail
  ],
)

// --------------------------------------------------------- shared styling ----

#let house_rules(body) = {
  set par(
    justify: true,
    leading: body_lead,
    spacing: body_space,
    linebreaks: "optimized",
    first-line-indent: 0pt,
  )
  set text(
    hyphenate: true,
    costs: (hyphenation: 100%, runt: 140%, widow: 320%, orphan: 320%),
  )
  set list(indent: 2pt, body-indent: 0.62em, spacing: 0.82em, marker: text(fill: accent)[#sym.bullet])
  set enum(indent: 2pt, body-indent: 0.62em, spacing: 0.82em)

  // Tables: no vertical rules, hairline row separators, a heavier rule under the
  // repeating header, and generous cell padding. Cell text is set ragged right;
  // justifying a 1.2in column is what produced the rivers in earlier drafts.
  show table: set par(justify: false, leading: 0.6em, spacing: 0.65em)
  show table: set text(size: 9.7pt, hyphenate: false)
  show table.cell.where(y: 0): set text(weight: "bold", size: 9.1pt, fill: accent)
  // Never split a row across a page: a stranded continuation cell reads as an error.
  set table.cell(breakable: false)
  set table(
    inset: (x: 7pt, y: 6pt),
    align: (_, _) => left + top,
    fill: (_, y) => if y == 0 { pale } else { none },
    stroke: (_, y) => (
      left: none,
      right: none,
      bottom: none,
      top: if y == 0 { 0.8pt + accent } else if y == 1 { 0.6pt + rule } else { 0.4pt + rule },
    ),
  )
  // Closing rule for the table, since a set rule cannot see the final row.
  show table: it => block(
    above: 12pt,
    below: 12pt,
    stroke: (bottom: 0.8pt + accent),
    it,
  )

  show heading.where(level: 1): it => block(above: 18pt, below: 8pt, sticky: true, width: 100%)[
    #sc_label(it.body, size: 12.5pt, fill: accent, weight: "bold", tracking: 0.05em)
    #v(5pt, weak: true)
    #line(length: 100%, stroke: 0.7pt + rule)
  ]
  show heading.where(level: 2): it => block(above: 14pt, below: 5pt, sticky: true)[
    #text(size: 11.2pt, weight: "bold", fill: ink)[#it.body]
  ]
  show heading.where(level: 3): it => block(above: 12pt, below: 4pt, sticky: true)[
    #text(size: 10.5pt, weight: "semibold", style: "italic", fill: ink)[#it.body]
  ]
  show link: set text(fill: accent)
  show strong: set text(weight: "bold", fill: ink)

  body
}

#let running_head(meta, size: 7.4pt) = {
  block(width: 100%, above: 0pt, below: 0pt, {
    grid(
      columns: (1fr, auto),
      align: (left + bottom, right + bottom),
      text(size: size, weight: "bold", fill: alarm, tracking: 0.08em)[#meta.review_label],
      text(size: size, fill: muted, tracking: 0.05em, smallcaps(meta.short_title)),
    )
    v(3pt, weak: false)
    line(length: 100%, stroke: 0.5pt + rule)
  })
}

#let folio(size: 8pt) = context align(center)[
  #text(size: size, fill: muted, tracking: 0.06em, smallcaps[Page #counter(page).display("1")])
]

// ------------------------------------------------------------- documents ----

#let filing(meta, body) = {
  set document(title: meta.title, author: meta.author)
  set page(
    paper: "us-letter",
    margin: (left: 1.05in, right: 1.05in, top: 0.85in, bottom: 0.72in),
    header: running_head(meta),
    header-ascent: 26%,
    footer: folio(),
    footer-descent: 34%,
  )
  set text(font: body_font, size: body_size, fill: ink, lang: "en", region: "US")

  show: house_rules

  align(center)[
    #v(2pt)
    #sc_label(meta.document_kind, size: 8.6pt, fill: accent, weight: "bold", tracking: 0.08em)
    #v(15pt)
    #block(width: 86%)[
      #par(justify: false, leading: 0.54em, linebreaks: "optimized")[
        #text(size: 20pt, weight: "bold", fill: ink, hyphenate: false)[#meta.title]
      ]
    ]
    #v(11pt)
    #line(length: 2.1in, stroke: 0.9pt + accent)
    #v(11pt)
    #par(justify: false, leading: 0.5em)[
      #text(size: 10.8pt, style: "italic", fill: muted, hyphenate: false)[#meta.subtitle]
    ]
  ]

  v(18pt)
  block(
    width: 100%,
    stroke: (top: 0.7pt + rule, bottom: 0.7pt + rule),
    inset: (top: 10pt, bottom: 10pt),
    {
      set par(justify: false, leading: 0.66em, spacing: 0pt)
      grid(
        columns: (1.18in, 1fr),
        column-gutter: 12pt,
        row-gutter: 7pt,
        sc_label("Proceeding"), [#meta.proceeding],
        sc_label("Identifiers"), [#text(number-width: "tabular")[#meta.identifiers]],
        sc_label("Submitted by"), [#meta.author],
        sc_label("Affiliation"), [#meta.affiliation],
        sc_label("Public contact"), [#meta.contact],
        sc_label("Draft date"), [#meta.draft_date],
      )
    },
  )

  review_note[
    *Public-review draft.* This document has not been filed, submitted, or approved
    by either Commission. Replace all bracketed identity fields; obtain appropriate
    legal and privacy review; recheck the live docket; and remove material that
    should not become permanently public. This is technical and policy analysis,
    not a legal opinion or a request for approval of a product, facility, or deployment.
  ]

  v(4pt)
  body
}

#let cover_filing(meta, body) = {
  set document(title: meta.title, author: meta.author)
  set page(
    paper: "us-letter",
    margin: (left: 0.95in, right: 0.95in, top: 0.74in, bottom: 0.64in),
    header: running_head(meta, size: 7.2pt),
    header-ascent: 24%,
    footer: folio(size: 7.6pt),
    footer-descent: 30%,
  )
  set text(font: body_font, size: 9.9pt, fill: ink, lang: "en", region: "US")

  show: house_rules
  set par(leading: 0.64em, spacing: 1.06em)
  set enum(indent: 2pt, body-indent: 0.6em, spacing: 0.6em)
  show heading.where(level: 1): it => block(above: 11pt, below: 4pt, sticky: true)[
    #sc_label(it.body, size: 10pt, fill: accent, weight: "bold", tracking: 0.06em)
  ]

  align(center)[
    #sc_label(meta.document_kind, size: 7.8pt, fill: accent, weight: "bold", tracking: 0.08em)
    #v(9pt)
    #block(width: 92%)[
      #par(justify: false, leading: 0.5em, linebreaks: "optimized")[
        #text(size: 16.5pt, weight: "bold", fill: ink, hyphenate: false)[#meta.title]
      ]
    ]
    #v(8pt)
    #line(length: 1.8in, stroke: 0.9pt + accent)
  ]

  v(12pt)
  block(
    width: 100%,
    stroke: (top: 0.7pt + rule, bottom: 0.7pt + rule),
    inset: (top: 7pt, bottom: 7pt),
    {
      set par(justify: false, leading: 0.62em, spacing: 0pt)
      grid(
        columns: (1.0in, 1fr),
        column-gutter: 11pt,
        row-gutter: 4.5pt,
        sc_label("Proceeding", size: 8pt), [#meta.proceeding],
        sc_label("Submitted by", size: 8pt), [#meta.author],
        sc_label("Affiliation", size: 8pt), [#meta.affiliation],
        sc_label("Public contact", size: 8pt), [#meta.contact],
        sc_label("Draft date", size: 8pt), [#meta.draft_date],
      )
    },
  )

  block(
    width: 100%,
    fill: warning,
    stroke: (left: 2.2pt + amber),
    inset: (left: 10pt, right: 11pt, top: 6.5pt, bottom: 6.5pt),
    radius: (right: 2pt),
    above: 11pt,
    below: 12pt,
    {
      set par(justify: false, leading: 0.66em)
      [*Review draft - not filed.* Identity, privacy, legal, source, and live-docket
      review remain required before filing.]
    },
  )

  body
}
