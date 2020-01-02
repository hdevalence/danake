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
inner-product identities can be used to rearrange terms.

Pedersen commitments are written as
\\[
    \operatorname{Com}(v) = \operatorname{Com}(v, {\widetilde{v}}) = v \cdot B + {\widetilde{v}} \cdot {\widetilde{B}},
\\]
where \\(B\\) and \\(\widetilde B\\) are the Pedersen generators used for the
values and blinding factors, respectively.  The blinding factor for the
value \\(v\\) is denoted by \\(\widetilde v\\), so that it is clear which
blinding factor corresponds to which value, and write
\\(\operatorname{Com}(v)\\) instead of \\(\operatorname{Com}(v,
\widetilde v)\\) for brevity.

