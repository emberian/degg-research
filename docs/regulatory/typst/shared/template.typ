#let ink = rgb("#172033")
#let muted = rgb("#586174")
#let rule = rgb("#c8cfda")
#let accent = rgb("#235789")
#let pale = rgb("#eef4f9")
#let warning = rgb("#fff5dc")

#let note_ref(number) = super(text(size: 7pt, fill: accent, str(number)))

#let review_note(body) = block(
  width: 100%,
  fill: warning,
  stroke: (left: 2.5pt + rgb("#c78416")),
  inset: (x: 11pt, y: 8pt),
  radius: 2pt,
  breakable: true,
  body,
)

#let key_point(title, body) = block(
  width: 100%,
  fill: pale,
  stroke: 0.7pt + rgb("#b9cadd"),
  inset: 11pt,
  radius: 3pt,
  breakable: false,
)[
  #text(weight: "bold", fill: accent)[#title]
  #v(3pt)
  #body
]

#let source_entry(number, title, url, detail) = {
  block(breakable: true)[
    #text(weight: "bold", fill: accent)[#number.] #link(url)[#title]. #detail
  ]
  v(5pt)
}

#let filing(meta, body) = {
  set document(title: meta.title, author: meta.author)
  set page(
    paper: "us-letter",
    margin: (left: 0.88in, right: 0.82in, top: 0.78in, bottom: 0.72in),
    header: align(left)[
      #text(size: 7.6pt, weight: "medium", fill: muted)[
        #meta.review_label  |  #meta.short_title
      ]
    ],
    footer: context align(center)[
      #text(size: 7.5pt, fill: muted)[Page #counter(page).display("1")]
    ],
  )
  set text(font: "Libertinus Serif", size: 10.4pt, fill: ink, lang: "en")
  set par(justify: true, leading: 0.62em, first-line-indent: 0pt)
  set list(indent: 15pt, body-indent: 6pt, spacing: 3pt)
  set enum(indent: 18pt, body-indent: 6pt, spacing: 3pt)
  set table(stroke: 0.55pt + rule, inset: 5pt)

  show heading.where(level: 1): it => block(above: 18pt, below: 7pt, breakable: false)[
    #text(size: 15pt, weight: "bold", fill: accent)[#it.body]
    #v(3pt)
    #line(length: 100%, stroke: 0.8pt + rule)
  ]
  show heading.where(level: 2): it => block(above: 12pt, below: 5pt, breakable: false)[
    #text(size: 12pt, weight: "bold", fill: ink)[#it.body]
  ]
  show heading.where(level: 3): it => block(above: 9pt, below: 4pt, breakable: false)[
    #text(size: 10.5pt, weight: "bold", style: "italic", fill: ink)[#it.body]
  ]
  show link: set text(fill: accent)

  align(center)[
    #v(7pt)
    #text(size: 8pt, weight: "bold", fill: accent)[#meta.document_kind]
    #v(12pt)
    #par(justify: false, leading: 0.5em)[
      #text(size: 21.5pt, weight: "bold", fill: ink, hyphenate: false)[#meta.title]
    ]
    #v(10pt)
    #line(length: 76%, stroke: 1pt + accent)
    #v(10pt)
    #text(size: 11pt, fill: muted)[#meta.subtitle]
  ]

  v(18pt)
  grid(
    columns: (1.2in, 1fr),
    column-gutter: 10pt,
    row-gutter: 5pt,
    [*Proceeding*], [#meta.proceeding],
    [*Identifiers*], [#meta.identifiers],
    [*Submitted by*], [#meta.author],
    [*Affiliation*], [#meta.affiliation],
    [*Public contact*], [#meta.contact],
    [*Draft date*], [#meta.draft_date],
  )

  v(15pt)
  review_note[
    *Public-review draft.* This document has not been filed, submitted, or approved
    by either Commission. Replace all bracketed identity fields; obtain appropriate
    legal and privacy review; recheck the live docket; and remove material that
    should not become permanently public. This is technical and policy analysis,
    not a legal opinion or a request for approval of a product, facility, or deployment.
  ]

  v(18pt)
  body
}

#let cover_filing(meta, body) = {
  set document(title: meta.title, author: meta.author)
  set page(
    paper: "us-letter",
    margin: (left: 0.78in, right: 0.74in, top: 0.62in, bottom: 0.58in),
    header: align(left)[
      #text(size: 7.4pt, weight: "medium", fill: muted)[
        #meta.review_label  |  #meta.short_title
      ]
    ],
    footer: context align(center)[
      #text(size: 7.3pt, fill: muted)[Page #counter(page).display("1")]
    ],
  )
  set text(font: "Libertinus Serif", size: 10pt, fill: ink, lang: "en")
  set par(justify: true, leading: 0.54em, first-line-indent: 0pt)
  set enum(indent: 16pt, body-indent: 5pt, spacing: 2.3pt)
  show heading.where(level: 1): it => block(above: 7pt, below: 3pt, breakable: false)[
    #text(size: 11pt, weight: "bold", fill: accent)[#it.body]
  ]
  show link: set text(fill: accent)

  align(center)[
    #v(2pt)
    #text(size: 7.5pt, weight: "bold", fill: accent)[#meta.document_kind]
    #v(5pt)
    #par(justify: false, leading: 0.5em)[
      #text(size: 18pt, weight: "bold", fill: ink, hyphenate: false)[#meta.title]
    ]
    #v(5pt)
    #line(length: 70%, stroke: 0.9pt + accent)
  ]

  v(7pt)
  grid(
    columns: (0.92in, 1fr),
    column-gutter: 8pt,
    row-gutter: 2.5pt,
    [*Proceeding*], [#meta.proceeding],
    [*Submitted by*], [#meta.author],
    [*Affiliation*], [#meta.affiliation],
    [*Public contact*], [#meta.contact],
    [*Draft date*], [#meta.draft_date],
  )

  v(6pt)
  block(
    width: 100%,
    fill: warning,
    stroke: (left: 2.2pt + rgb("#c78416")),
    inset: (x: 8pt, y: 5pt),
    radius: 2pt,
  )[
    *Review draft - not filed.* Identity, privacy, legal, source, and live-docket
    review remain required before filing.
  ]

  v(7pt)
  body
}
