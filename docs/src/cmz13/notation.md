# Notation

The notation in this document differs from the notation in the CMZ13
paper but has the important advantages that the type of each variable is
readily visible in its notation, and that the notation in the abstract
description matches the notation in the implementation.  

\\(\\mathbb G\\) denotes an abelian group of prime order \\(p\\).  As it
is abelian, it is written additively.  Group elements are denoted by
upper-case letters \\(A, B, \ldots\\), while scalars are denoted by
lower-case letters \\(a, b, \ldots\\); boldface letters \\(\mathbf a,
\mathbf G\\) denote a vector of elements (in these cases, a vector of
scalars and a vector of group elements, respectively).

The inner product of two vectors is denoted by \\(\langle-,-\rangle\\).
Notice that with this notation, a multiscalar multiplication is written
as \\(\langle\mathbf a, \mathbf G\rangle\\), and all the usual
inner-product identities can be used to rearrange terms.  The
concatenation of two vectors is written \\( \mathbf x || \mathbf y \\).

The notation \\( P \xleftarrow{\\$} \\mathbb G\\) means that \\(P\\)
should be selected uniformly at random from the set \\(\mathbb G\\).

Pedersen commitments are written as
\\[
    \operatorname{Com}(v) 
  = \operatorname{Com}(v, {\widetilde{v}})
  = v \cdot B + {\widetilde{v}} \cdot {\widetilde{B}},
\\]
where \\(B\\) and \\(\widetilde B\\) are the Pedersen generators used for the
values and blinding factors, respectively.  The blinding factor for the
value \\(v\\) is denoted by \\(\widetilde v\\), so that it is clear which
blinding factor corresponds to which value, and write
\\(\operatorname{Com}(v)\\) instead of \\(\operatorname{Com}(v,
\widetilde v)\\) for brevity.

The ElGamal encryption of a group element \\(A\\) to the public key \\(D\\)
is written as
\\[
  \operatorname{Enc}\_D(A) = 
  (rB, A + rD),
\\]
where \\(r\\) is the randomness used for the encryption and \\(B\\) is the
generator used for the public key \\(D\\).  The individual components are denoted by
\\(
  \operatorname{Enc}\_D(A)\_0
\\)
and 
\\(
  \operatorname{Enc}\_D(A)\_1
\\).

Non-interactive Schnorr proofs are denoted by the following
Camenisch-Stadler-like notation, which matches the notation used in the `zkp`
crate used to automatically derive the proof implementations:
\\[
\begin{aligned}
\operatorname{PK}&\\{ \\\\
    &\mathtt{CorrectElGamal}, \\\\
    &(d, (r\_i, m\_i)\_{i \in \mathcal H}), \\\\
    &(D, (E\_{i,0}, E\_{i,1})\_{i \in \mathcal H}), \\\\
    &(B) \\; : \\\\
    &(E\_{i,0}, E\_{i,1}) = (r\_i B, m\_i B + r\_i D) \\; \forall i \in \mathcal H \\\\
\\}
\end{aligned}
\\]

Here the first line is an ASCII domain separation label used to identify the
proof statement; the second line is a list of secret scalar variables
(witness data); the third line is a list of public group element variables
unique to each proof instance; the fourth line is a list of public group
element variables common across all proof instances; the remaining lines are
the statements to be proved. Distinguishing public variables which are common
across all proof instances from ones which are unique to each proof instance
allows a derived implementation to take advantage of precomputation and
provide batch verification.