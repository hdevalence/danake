# Extensions to CMZ13

Danake uses the CMZ13 construction mostly unmodified, except for a few
minor extensions detailed explicitly in this section.

1. While the CMZ13 paper describes proofs of correctness for each
protocol step, Danake uses [Merlin] transcripts to chain these
incremental proofs into proofs of correctness of an entire protocol run.

2. Danake uses _nullifiers_ to provide credentials with use-once
("affine") semantics, allowing the encoding of more complex state
machines at the cost of requiring the issuer to maintain a
nullifier set.

3. Danake introduces a notion of _credential epochs_, which provide
continual key rotation and nullifier set pruning.

[Merlin]: https://merlin.cool
