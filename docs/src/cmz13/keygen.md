# Key Generation

The following procedure defines common parameters for all credentials:

* **Setup**.  Choose \\(\mathbb G\\) a group of prime order \\(p\\).
  \\( \mathbb G \\) should be equipped with a hash-to-group method
  suitable for choosing orthogonal generators.  Select orthogonal
  generators \\(B, \widetilde B\\).

To issue credentials, the issuer generates *issuance secrets*, which are
used to create and verify credentials, and *issuance parameters*, which
commit to the issuance secrets and are used by clients to verify that
their credentials are issued with respect to the same issuance secrets
as all other clients, preventing key partitioning attacks.

* **Key Generation**.  Choose a \\(\\mathsf{MAC\_{GGM}}\\) secret
  \\( 
  \mathbf x = (x\_0, x\_1, \ldots, x\_n) \xleftarrow{\\$} \mathbb F\_p^{n+1}
  \\).
  Also select a blinding factor
  \\(
  \widetilde x\_0 \xleftarrow{\\$} \mathbb F\_p
  \\),
  then form the Pedersen commitment
  \\(
  \operatorname{Com}(x\_0) = x\_0 B + \widetilde x\_0 \widetilde B
  \\)
  and compute
  \\(
  X\_i = x\_i \widetilde B
  \\) for \\( i = 1, \ldots, n\\).

  The *issuance secrets* are \\((\mathbf x, \widetilde x_0)\\) and the *issuance parameters* are \\((\mathbf X, \operatorname{Com}(x\_0))\\).
  
FIXME: rewrite to make pedersen commitment structure more clear?