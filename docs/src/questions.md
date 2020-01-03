# Unresolved Questions


## Parameter Transparency

Ideally, the issuer should commit issuance parameters to a transparency
log that clients can check independently -- could the issuer bundle
parameters with proofs-of-inclusion?

## Transport Reliability

Currently Danake assumes a reliable transport; if a wallet transaction
is dropped and the response is never received by the client, the client
may be left hanging (having spent their previous token).  There are a
few workarounds for this (e.g., allowing clients to re-download the
results of issuance?) and one of them should be chosen.

## Interaction with HTTP semantics

Ideally, Danake payments should be fast enough to allow billing of every
HTTP request.  One form this could take would be a `danake-proxy` that
sits in front of a web application and inspects HTTP headers.  How can
this be made to interact properly with HTTP semantics around
idempotency, etc?  How can this be implemented on the client?
