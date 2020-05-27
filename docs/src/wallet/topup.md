# Topup

Wallet topup allows a client to add credit to their wallet credential,
revealing the credit increase (so that the issuer can check that the increase
is in line with application-specific policy) but hiding the old and new
balances.

1. **Client**. Given the credit topup amount \\(c\\) as well as credential
attributes \\((w,n)\\) and tag \\((P\_0,Q\_0)\\) for a previously-issued
wallet credential, the client proceeds as follows, simultaneously preparing a
presentation of the previous wallet credential and a request for blinded
issuance of a new credential with balance \\(w' = w + c\\). The client:

    1. Uses the current time and the epoch duration to determine the current
    epoch index. If the existing credential's issuance parameters are not the
    primary issuance parameters for the current epoch, perform a rollover to
    the current parameters and restart the protocol.

    2. Re-randomizes the tag, choosing \\(t \xleftarrow{\\$} \mathbb F\_p\\)
    and computing \\((P, Q) \gets (t P\_0, t Q\_0)\\).

    3. Chooses randomness
    \\(
        \widetilde w \xleftarrow{\\$} \mathbb F\_p
    \\)
    and computes
    \\(
        \operatorname{Com}(w) \gets w P + \widetilde w \widetilde B
    \\)
    and
    \\(
        \operatorname{Com}(w') \gets w' P + \widetilde w \widetilde B
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
        \operatorname{Enc}\_D(w' B) \gets (r\_w  B, (w' + r\_w d)B)
    \\]
    and
    \\[
    \operatorname{Enc}\_D(n' B) \gets (r\_n B, (n' + r\_n d)B).
    \\]

    9.  Forms the following proof, combining credential presentation and
    blinded issuance:
    \\[
    \begin{aligned}
    \pi &\gets \operatorname{PK}\\{ \\\\
        &\mathtt{wallet::topup::client}, \\\\
        &(d, w, w', \widetilde w, n', r\_Q, r\_w, r\_n), \\\\
        &(
            D, 
            \operatorname{Enc}\_D(w' B),
            \operatorname{Enc}\_D(n' B),
            P,
            V,
            \operatorname{Com}(w),
            \operatorname{Com}(w')
        ), \\\\
        &(B, \widetilde B, \mathbf X) \\; : \\\\
        &D = d B \\\\
        &\operatorname{Enc}\_D(n' B) = (r\_n B, n' B + r\_n D) \\\\
        &\operatorname{Enc}\_D(w' B) = (r\_w B, w' B + r\_w D) \\\\
        &\operatorname{Com}(w) = w P + \widetilde w \widetilde B \\\\
        &\operatorname{Com}(w') = w' P + \widetilde w \widetilde B \\\\
        & V = \widetilde w X\_1 - r\_Q B \\\\
    \\}.
    \end{aligned}
    \\]
    The client retains the transcript state for the next step.

    10. Using the same transcript, forms a rangeproof
    \\(\rho\\) proving that \\(\operatorname{Com}(w')\\) commits to a
    value in range \\([0,2\^{64})\\), and retains the transcript state.

    10. Sends the epoch index, the old nullifier \\(n\\),
    \\(D\\),
    \\(\operatorname{Enc}(n'B)\\),
    \\(\operatorname{Enc}(w'B)\\),
    \\(\operatorname{Com}(w)\\),
    \\(P\\),
    \\(C\_Q\\),
    \\(\pi\\),
    and \\(\rho\\)
    to the issuer.

2. **Issuer**.  The issuer proceeds as follows. The issuer:

    1. Checks the requested credit amount \\(c\\) according to
    application policy.

    2. Checks that the issuance parameters for the epoch index
    selected by the client are in Active or Primary state.

    3. Checks whether the nullifier \\(n\\) is in the wallet nullifier
    set for the selected epoch, rejecting the request if it is present and
    adding it to the nullifier set if it is not present.

    4. Computes \\(V\\) as
    \\[
        V \gets (x\_0 + x\_2 n) B + x\_1 \operatorname{Com}(w) - C\_Q.
    \\]

    5. Computes
    \\(
    \operatorname{Com}(w') \gets \operatorname{Com}(w) + cP
    \\).

    6. Verifies the proof \\(\pi\\) and retains the transcript state.

    7. Verifies the proof \\(\rho\\) and retains the transcript state.

    8. Selects
    \\( b \xleftarrow{\\$} \mathbb F\_p \\)
    and computes
    \\( P \gets bB \\).

    9.  Selects
    \\( r \xleftarrow{\\$} \mathbb F\_p \\)
    to compute
    \\[
    \operatorname{Enc}\_D(Q) \gets (rB, x\_0 P + rD) + 
    b x\_1 \operatorname{Enc}\_D(w'B) +
    b x\_2 \operatorname{Enc}\_D(n'B).
    \\]
    
    10.  The issuer forms the proof
    \\[
    \begin{aligned}
    \pi &\gets \operatorname{PK}\\{ \\\\
        &\mathtt{wallet::topup::issuer}, \\\\
        &(
            b, 
            r, 
            \mathbf x,
            \widetilde x\_0,
            t\_1, 
            t\_2
        ), \\\\
        &(
            P,
            D, 
            \operatorname{Enc}\_D(w'B),
            \operatorname{Enc}\_D(n'B),
            \operatorname{Enc}\_D(Q),
            T\_1,
            T\_2
        ), \\\\
        &(\mathbf X, B, \widetilde B) \\; : \\\\
        & X\_0 = x\_0 B + \widetilde x\_0 \widetilde B, \\;
            X\_1 = x\_1 \widetilde B, \\;
            X\_2 = x\_2 \widetilde B, \\\\
        & P = bB, \\\\
        & T\_1 = bX\_1', \\; T\_1 = t\_1 \widetilde B, \\\\
        & T\_2 = bX\_2', \\; T\_2 = t\_2 \widetilde B, \\\\
        & \operatorname{Enc}\_D(Q) =
            (rB, x\_0 P + rD) + 
            t\_1 \operatorname{Enc}\_D(w'B) +
            t\_2 \operatorname{Enc}\_D(n'B) \\\\
    \\}.
    \end{aligned}
    \\]
    This proof should be added to the transcript from step (2.7),
    chaining the issuer's proof onto the client's proofs.

    11.  The issuer sends \\(P\\), \\(\operatorname{Enc}\_D(Q)\\),
    \\(T\_1\\), \\(T\_2\\), and \\(\pi\\) to the client.

3. **Client**. The client processes the response as follows:

    1.  The client uses the transcript state from step (1.10) to verify
    \\(\pi\\).

    2.  The client decrypts \\(Q\\) by computing
    \\[
    Q \gets \operatorname{Enc}\_D(Q)\_1 - d \operatorname{Enc}\_D(Q)\_0.
    \\]
