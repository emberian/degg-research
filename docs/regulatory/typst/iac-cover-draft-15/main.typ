#import "../shared/template.typ": cover_filing
#import "metadata.typ": meta

#show: body => cover_filing(meta, body)

#include "body.typ"
#include "sources.typ"
