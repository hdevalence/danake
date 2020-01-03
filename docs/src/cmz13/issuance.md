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

Concretely, issuance proceeds as follows:

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

5. The server verifies the proof and optionally performs other policy checks related to issuance.

6. 

[^1]: Because the client knows their own ephemeral secret, assuming an
optimized fixed-base scalar multiplication is available, this can be
optimized as
\\[
   (E\_{i,0}, E\_{i,1}) \gets 
   (r\_i B, (m\_i + r\_i d)B) = (r\_i B, m\_i B + r\_i D).
\\]