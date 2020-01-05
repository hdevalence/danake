# Topup

Wallet topup allows a client to add credit to their wallet credential,
revealing the credit increase (so that the issuer can check that the increase
is in line with application-specific policy) but hiding the old and new
balances. To do this, the client requests blinded issuance of a new
credential with updated balance and a new nullifier, and proves that the
difference between old and new balances is the expected increase.

1. **Client**. Given the credit topup amount \\(c\\) as well as credential
attributes \\((w,n)\\) and tag \\((P\_0,Q\_0)\\) for a previously-issued
wallet credential, the client proceeds as follows, simultaneously preparing a
presentation of the previous wallet credential and a request for blinded
issuance of a new credential. The client:

    1. Uses the current time and the epoch duration to determine the current
    epoch index. If the existing credential's issuance parameters are not the
    primary issuance parameters for the current epoch, perform a rollover to
    the current parameters and restart the protocol.

    2. Re-randomizes the tag, choosing \\(t \xleftarrow{\\$} \mathbb F\_p\\)
    and computing \\((P, Q) \gets (t P\_0, t Q\_0)\\).

    3.  Computes 
    \\(
        \operatorname{Com}(w) = w P + \widetilde w \widetilde B
    \\)
    using randomness
    \\(
        \widetilde w \xleftarrow{\\$} \mathbb F\_p
    \\).

    4.  Commits to \\(Q\\) by choosing
    \\(
        r\_Q \xleftarrow{\\$} \mathbb F\_p
    \\)
    and computing
    \\(
        C\_Q = Q + r\_Q B
    \\).

    5.  Computes a correction term
    \\(
        V \gets \widetilde w X\_1 - rB
    \\).

    6.  Generates a random nullifier for the new credential,
    \\(
        n' \xleftarrow{\\$} \mathbb F\_p
    \\).

    7.  Generates an ephemeral ElGamal public key
    \\(
        d \xleftarrow{\\$} \mathbb F\_p
    \\)
    and computes the ephemeral public key
    \\(
        D \gets d B
    \\).

    8.  Generates randomness
    \\(
        r\_n, r\_w \xleftarrow{\\$} \mathbb F\_p
    \\)
    and computes
    \\[
        \operatorname{Enc}\_D(w' B) \gets (r\_w  B, (w + r\_w d)B)
    \\]
    and
    \\[
    \operatorname{Enc}\_D(n' B) \gets (r\_n B, (n' + r\_n d)B).
    \\]
