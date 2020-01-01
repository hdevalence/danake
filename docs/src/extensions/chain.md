# Chaining Proofs

Various parts of the CMZ13 construction make use of zero-knowledge
proofs to have, e.g., the issuer prove to the user that the credential
was issued with respect to the expected parameters, or the user prove to
the issuer that their blinded attributes were well-formed encryptions.

This is a specific instance of a more general paradigm: the use of
zero-knowledge proofs to be transformed from a setting with
possibly-malicious participants, who may not behove correctly, to
a setting with honest-but-curious participants, who perform the protocol
steps correctly, but might try to learn extra information.

To do this, as each participant performs their protocol step,
they also construct a zero-knowledge proof that they performed their
step correctly, then sends that proof along with the protocol message to
their counterparty, who can use it to check that the message was
correctly constructed.  The counterparty can then produce a proof that
they performed their step correctly, and so on.

The CMZ13 proofs used by Danake are instantiated using
[Merlin][merlin_site] to perform the Fiat-Shamir transformation,
modeling transcript using the duplex sponge construction.  One emergent
property of this design, as explained in the [Merlin blog
post][merlin_post], is that because the prover and verifier perform
identical transcript operations, they can interactively compose
intermediate proofs of correctness of each step into
proofs of correctness of the entire protocol, without requiring any
implementation changes to the proving functions.

This works as follows.  Both counterparties (in this case, the client
and issuer) maintain their own transcript states.  When one party proves
that they performed their step correctly, they mutate the state of their
transcript away from the state of their counterparty’s transcript.  But
when their counterparty verifies the proof, the counterparty’s
transcript state is mutated in the same way, bringing the two parties’
transcripts back into sync.  Now the roles reverse, with the next
proving phase mutating the transcript again, and the counterparty’s
verification synchronizing the transcripts.

Notice that the proof of each step is a noninteractive proof, but
because they’re made on the same transcript, they’re composed with all
of the proofs from all prior steps, forming a chain of verifications of
the entire protocol.

[merlin_site]: https://merlin.cool
[merlin_post]: https://medium.com/@hdevalence/merlin-flexible-composable-transcripts-for-zero-knowledge-proofs-28d9fda22d9a

