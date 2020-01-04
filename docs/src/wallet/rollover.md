# Wallet Rollover

To rollover a wallet credential to a new epoch, the client requests blinded
issuance of a new credential with the same wallet balance and a new
nullifier.

1. **Client**.  Given the credential attributes \\((w,n)\\) and tag \\((P\_0,Q\_0)\\)
    for a previously-issued wallet credential, the client proceeds as follows, simultaneously preparing a presentation of the previous wallet credential and a request for blinded issuance of a new credential.  The client:

    1.  Uses the current time and the epoch duration to determine
        the epoch index for the new credential, denoting by \\(\mathbf X\\) the
        issuance parameters for the current credential's epoch and by \\(\mathbf
        X'\\) the issuance parameters for the new credential's epoch.

    2.  Re-randomizes the tag, choosing
        \\(t \xleftarrow{\\$} \mathbb F\_p\\) and computing
        \\((P, Q) \gets (t P\_0, t Q\_0)\\).

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
        \operatorname{Enc}\_D(w B) \gets (r\_w  B, (w + r\_w d)B)
        \\]
        and
        \\[
        \operatorname{Enc}\_D(n' B) \gets (r\_n B, (n' + r\_n d)B).
        \\]

    9.  Forms the following proof, which combines credential presentation
        and blinded issuance statements:
        \\[
        \begin{aligned}
        \pi &\gets \operatorname{PK}\\{ \\\\
            &\mathtt{WalletRollover\\_Client}, \\\\
            &(d, w, \widetilde w, n', r\_Q, r\_w, r\_n), \\\\
            &(
                D, 
                \operatorname{Enc}\_D(w B),
                \operatorname{Enc}\_D(n' B),
                P,
                V,
                \operatorname{Com}(w)
            ), \\\\
            &(B, \widetilde B) \\; : \\\\
            &\operatorname{Enc}\_D(n' B) = (r\_n B, n' B + r\_n D) \\\\
            &\operatorname{Enc}\_D(w B) = (r\_w B, w B + r\_w D) \\\\
            &\operatorname{Com}(w) = w P + \widetilde w \widetilde B \\\\
            & V = \widetilde w X\_1 - r\_Q B \\\\
        \\}.
        \end{aligned}
        \\]
        The proof transcript should additionally be bound to the epoch indexes of the current epoch and of the requested epoch.  The client keeps the transcript state while awaiting a response.

    10. Sends the pair of epoch indices, 
        the old nullifier \\(n\\),
        \\(D\\),
        \\(\operatorname{Enc}(n'B)\\),
        \\(\operatorname{Enc}(wB)\\),
        \\(\operatorname{Com}(w)\\),
        \\(C\_Q\\),
        and \\(\pi\\) 
        to the issuer.
    
2. **Issuer**.  The issuer processes the request as follows.  The issuer:

    1.  Checks that the issuance parameters for the old epoch index
        specified by the client are in Active, Primary, or Rollover state, and
        that the issuance parameters for the new epoch index specified by the
        client are in the Active or Primary state.  The old parameters are denoted 
        by \\((\mathbf X, \mathbf x)\\) and the new parameters are denoted by
        \\((\mathbf X', \mathbf x')\\).

    2.  Checks whether the nullifier \\(n\\) is in the wallet nullifier set
        for the old epoch, rejecting the request if it is present
        and adding it to the nullifier set if it is not present.
    
    3.  Computes \\(V\\) using the old secrets as 
        \\[
            V \gets (x\_0 + x\_2 n) B + x\_1 \operatorname{Com}(w) - C\_Q.
        \\]

    4.  Verifies the proof \\(\pi\\) and saves the transcript state.
    
    5.  Selects
        \\( b \xleftarrow{\\$} \mathbb F\_p \\)
        and computes
        \\( P \gets bB \\).

    6.  Selects
        \\( r \xleftarrow{\\$} \mathbb F\_p \\)
        to compute
        \\[
        \operatorname{Enc}\_D(Q) \gets (rB, x\_0' P + rD) + 
        b x\_1' \operatorname{Enc}\_D(wB) +
        b x\_2' \operatorname{Enc}\_D(n'B)
        \\]
        using the new secrets.
    
    7.  The issuer forms the proof
        \\[
        \begin{aligned}
        \pi &\gets \operatorname{PK}\\{ \\\\
            &\mathtt{WalletRollover\\_Issuer}, \\\\
            &(
                b, 
                r, 
                \mathbf x,
                \widetilde x\_0,
                \mathbf x', 
                \widetilde x\_0', 
                t\_1, 
                t\_2
            ), \\\\
            &(
                P,
                D, 
                \operatorname{Enc}\_D(wB),
                \operatorname{Enc}\_D(n'B),
                \operatorname{Enc}\_D(Q),
                T\_1,
                T\_2
            ), \\\\
            &(\mathbf X, \mathbf X', B, \widetilde B) \\; : \\\\
            & X\_0 = x\_0 B + \widetilde x\_0 \widetilde B, \\;
                X\_1 = x\_1 \widetilde B, \\;
                X\_2 = x\_2 \widetilde B, \\\\
            & X\_0' = x\_0' B + \widetilde x\_0' \widetilde B, \\;
                X\_1' = x\_1' \widetilde B, \\;
                X\_2' = x\_2' \widetilde B, \\\\
            & P = bB, \\\\
            & T\_1 = bX\_1', \\; T\_1 = t\_1 \widetilde B, \\\\
            & T\_2 = bX\_2', \\; T\_2 = t\_2 \widetilde B, \\\\
            & \operatorname{Enc}\_D(Q) =
                (rB, x\_0' P + rD) + 
                t\_1 \operatorname{Enc}\_D(wB) +
                t\_2 \operatorname{Enc}\_D(n'B) \\\\
        \\}.
        \end{aligned}
        \\]
        This proof should be added to the transcript from step (2.4), chaining
        the issuer's proof onto the client's proof.  
        
    8.  The issuer sends \\(P\\), \\(\operatorname{Enc}\_D(Q)\\), \\(T\_1\\), \\(T\_2\\), and
        \\(\pi\\) to the client.
    
3. **Client**.  The client processes the response as follows:

    1.  The client uses the transcript state from step (1.9) to verify \\(\pi\\).

    2.  The client decrypts \\(Q\\) by computing
        \\[
        Q \gets \operatorname{Enc}\_D(Q)\_1 - d \operatorname{Enc}\_D(Q)\_0.
        \\]
