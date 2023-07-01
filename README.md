# smacktivity

(eventually) a single-user activitypub implementation and activitypub library
for rust


## TODO

- [ ] json-ld stuff
    - [X] read @context
    - [ ] verify @context (see https://www.w3.org/ns/activitystreams#h-introduction)
    - [ ] extra contexts on objects
- [ ] better object properties
    - [ ] contentMap (rdf:langString)
    - [ ] nameMap (rdf:langString)
    - [ ] duration (xsd:duration type)
    - [ ] hreflang (BCP47 https://lib.rs/crates/language-tags)
    - [ ] end_time (xsd:dateTime)
    - [ ] published (xsd:dateTime)
    - [ ] start_time (xsd:dateTime)
    - [X] rel (RFC5988 https://datatracker.ietf.org/doc/html/rfc5988)
    - [ ] summaryMap (rdf:langString)
    - [X] units (string enum https://www.w3.org/TR/activitystreams-vocabulary/#dfn-units)
    - [ ] updated (xsd:dateTime)
    - [ ] deleted (xsd:dateTime)
- [ ] object hierarchy?
    - probably won't do this one. doesn't seem super useful for implementors.
- [ ] security
    - [ ] HTTP Signature in requests (https://datatracker.ietf.org/doc/html/draft-cavage-http-signatures)
    - [ ] Digest header
    - [ ] UI authentication
    - [ ] publicKey property (https://w3c-ccg.github.io/security-vocab/#publicKey)
- [ ] webfinger (https://datatracker.ietf.org/doc/html/rfc7033)
