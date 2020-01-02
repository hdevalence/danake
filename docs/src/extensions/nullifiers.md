# Affine Credentials

One application of a keyed-verification credential system, anonymous or
otherwise, is secure client-side encoding of a state machine.  Rather
than having a server maintain state for each client in a central
database, a server can offload state to its clients, relying on the
security of the credential to prevent clients from tampering with their
state.

This is particularly interesting when the credential system is an
anonymous credential system, because clients can prove statements about
their (previously-authenticated) state in zero knowledge, rather than
revealing the state itself.

However, usefully encoding a state machine requires creating credentials
that can only be presented once.  Borrowing terminology from
[substructural type systems][wiki], these could be called "affine
credentials".

Extending CMZ13 credentials to have use-once semantics is easy, by
adding a nullifier attribute to a credential.  During issuance, clients
generate a random nullifier and requests blinded issuance of a
credential with that nullifier.  During presentation, clients reveal the
nullifier, allowing the issuer to perform a set membership query.

This approach gives a hard guarantee that credentials are not presented
twice, at the cost of requiring the issuer to maintain a nullifier set.
However, the nullifier set can be continually pruned using the epoch
system described in the next section.

[wiki]: https://en.wikipedia.org/wiki/Substructural_type_system


