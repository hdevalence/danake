# Proof Chaining

Various parts of the CMZ13 construction make use of zero-knowledge
proofs to have, e.g., the issuer prove to the user that the credential
was issued with respect to the expected parameters, or the user prove to
the issuer that their blinded attributes were well-formed encryptions.

This is a specific instance of a more general paradigm: the use of
zero-knowledge proofs to transform a protocol secure in an
honest-but-curious setting (where all participants can be assumed to
follow the protocol steps correctly) to one secure in a byzantine
setting (where participants can perform arbitrary steps).

XXX add ratcheting description via merlin -- rather than just doing
proofs one at a time, we can chain together all proofs in a single
"transaction", so that, e.g., the issuer's correct-issuance proof is
bound to the user's correct-encryption proof, and what's really
happening is that the issuer and the user are jointly producing proofs
of correct execution of an entire protocol run.
