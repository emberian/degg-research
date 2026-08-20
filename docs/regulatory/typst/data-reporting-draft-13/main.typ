#import "../shared/template.typ": filing
#import "metadata.typ": meta

#show: body => filing(meta, body)

#include "body.typ"
#include "sources.typ"
