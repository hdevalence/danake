# Issuance

Following CMZ13, this description of blinded issuance denotes the set of
hidden attribute indexes by \\(\mathcal H \subseteq \\{1,\ldots,n\\}\\). At a
high level, the issuer computes a MAC on the requested attributes, then
returns it, together with a proof that it was computed with respect to the
expected issuance parameters. To handle blinded attributes, the client
generates an ephemeral ElGamal key, encrypts the attributes to be blinded,
and proves that the encryptions were well-formed. Because ElGamal encryptions
are homomorphic, a MAC on encrypted attributes can be converted into an
encryption of a MAC on the plaintext attributes.

Issuance is an online protocol with the following steps.

1. **Client**.  The client proceeds as follows.
    1. The client generates an ephemeral ElGamal secret
       \\(
       d \xleftarrow{\\$} \mathbb F\_p
       \\)
       and computes the ephemeral public key
       \\(
       D \gets d B
       \\).
    
    2. For each blinded attribute \\(m\_i\\) indexed by \\(i \in \mathcal H\\),
       the client chooses
       \\(
       r\_i \xleftarrow{\\$} \mathbb F\_p
       \\)
       and computes 
       \\(
       (E\_{i,0}, E\_{i,1}) \gets \operatorname{Enc}\_D(m\_i B)
       = (r\_i B, m\_i B + r\_i D)
       \\).[^1]
    
    3. The client proves that the encryptions were well-formed:
       \\[
       \begin{aligned}
       \pi &\gets \operatorname{PK}\\{ \\\\
           &\mathtt{CorrectElGamal}, \\\\
           &(d, (r\_i, m\_i)\_{i \in \mathcal H}), \\\\
           &(D, (E\_{i,0}, E\_{i,1})\_{i \in \mathcal H}), \\\\
           &(B) \\; : \\\\
           &(E\_{i,0}, E\_{i,1}) = (r\_i B, m\_i B + r\_i D) \\; \forall i \in \mathcal H \\\\
       \\}
       \end{aligned}
       \\]
    
    4. The client sends \\(D\\), \\((m\_i)\_{i \in \mathcal H}\\), 
       \\((E\_{i,0}, E\_{i,1})\_{i \in \mathcal H}\\) and \\(\pi\\) to the server.
    
2. **Issuer**.  The issuer verifies the client's proof and optionally
   performs other policy checks related to issuance.  Now 
   the issuer would like to select
   \\( P \xleftarrow{\\$} \mathbb G \\)
   and compute
   \\( Q \gets \langle \mathbf x, (1) || \mathbf m \rangle P \\),
   but this cannot be done directly as the attributes
   \\( (m\_i)\_{i \in \mathcal H} \\)
   are not available.  Instead, the issuer will compute
   \\( \operatorname{Enc}\_D(Q) \\) as follows,
   decomposing \\(Q\\) as \\( Q = Q\_c + Q\_b\\) and
   considering the contributions from cleartext attributes \\(Q\_c\\)
   and blinded attributes \\(Q\_b\\) separately:
    1. The issuer selects
       \\( p \xleftarrow{\\$} \mathbb F\_p \\)
       and computes
       \\( P \gets pB \\).
    2. The issuer computes[^2] a partial MAC on the cleartext attributes
       \\[
       %Q\_c \gets \langle 
       %  (x\_0) || (x\_i)\_{i \not\in \mathcal H},
       %  (1) || (m\_i)\_{i \not\in \mathcal H},
       %\rangle P.
       Q\_c \gets \Big(
       x\_0 + \sum\_{i \not\in \mathcal H} x\_i m\_i
       \Big) P.
       \\]
    3. The issuer selects randomness
       \\( r \xleftarrow{\\$} \mathbb F\_p \\)
       to compute
       \\[
       \operatorname{Enc}\_D(Q\_c)
       \gets
       (rB, Q\_c + rD).
       \\]
    4. The issuer uses
       \\(E\_i = \operatorname{Enc}\_D(m\_i B) \\)
       to compute
       \\[
       \operatorname{Enc}\_D(Q\_b)
       \gets
       \sum\_{i \in \mathcal H}
         p x\_i \operatorname{Enc}\_D(m\_i B).
       \\]
    5. The issuer computes
       \\[
       \operatorname{Enc}\_D(Q)
       \gets
       \operatorname{Enc}\_D(Q\_c) +
       \operatorname{Enc}\_D(Q\_b).
       \\]
    6. The issuer proves that it performed its steps correctly:
       \\[
       \begin{aligned}
       \pi &\gets \operatorname{PK}\\{ \\\\
           &\mathtt{CorrectBlindIssuance}, \\\\
           &(), \\\\
           &(), \\\\
           &() \\; : \\\\
           & \ldots \\\\
       \\}
       \end{aligned}
       \\]
    7. The issuer sends \\(P\\), \\(\operatorname{Enc}\_D(Q)\\), and
       \\(\pi\\) to the client.
    
3. **Client**. The client verifies the issuer's proof using the expected
issuance parameters and decrypts \\(\operatorname{Enc}\_D(Q)\\) to
obtain the tag \\((P,Q)\\).


[^1]: Because the client knows their own ephemeral secret, assuming an
optimized fixed-base scalar multiplication is available, this can be
optimized as
\\[
   (E\_{i,0}, E\_{i,1}) \gets 
   (r\_i B, (m\_i + r\_i d)B) = (r\_i B, m\_i B + r\_i D).
\\]

[^2]: Since \\( P = pB \\), this can also be optimized as a fixed-base
scalar multiplication.
