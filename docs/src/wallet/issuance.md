# Wallet Issuance

1. **Client**.  The client proceeds as follows.

    1. The client uses the current time and the epoch duration to determine
        the current epoch index and appropriate issuance parameters.

    2.  The client generates a random nullifier 
        \\(
        n \xleftarrow{\\$} \mathbb F\_p
        \\).

    3.  The client generates an ephemeral ElGamal public key
        \\(
        d \xleftarrow{\\$} \mathbb F\_p
        \\)
        and computes the ephemeral public key
        \\(
        D \gets d B
        \\).

    4.  The client generates randomness
        \\(
        r \xleftarrow{\\$} \mathbb F\_p
        \\)
        and computes
        \\[
        \operatorname{Enc}\_D(n B) \gets (r B, (n + rd)B).
        \\]

    5.  The client forms the proof
        \\[
        \begin{aligned}
        \pi &\gets \operatorname{PK}\\{ \\\\
            &\mathtt{WalletIssuance\\_Client}, \\\\
            &(d, n, r), \\\\
            &(D, \operatorname{Enc}\_D(n B)), \\\\
            &(B) \\; : \\\\
            &\operatorname{Enc}\_D(n B) = (rB, nB + rD) \\\\
        \\}.
        \end{aligned}
        \\]
        The proof transcript should additionally be bound to the current
        epoch index and the expected issuance parameters.  The client 
        keeps the transcript state while awaiting a response.

    6.  The client sends the epoch index, \\(D\\),
        \\(\operatorname{Enc}(nB)\\), \\(\pi\\), and any other policy-dependent
        data relevant to the request to the issuer.
    
2. **Issuer**.  The issuer processes the request as follows.

    1.  The issuer checks that the issuance parameters for the epoch index
        specified by the client are in Active or Primary state.

    2.  The issuer checks the policy-dependent data specified by the client or
        performs other policy checks, determining the issuance amount 
        \\(0 \le w < 2\^{64}\\).

    3.  The issuer verifies the proof \\(\pi\\) and saves the transcript state.
    
    4.  The issuer selects
        \\( b \xleftarrow{\\$} \mathbb F\_p \\)
        and computes
        \\( P \gets bB \\).

    5.  The issuer computes 
        \\( Q\_c \gets (x\_0 + x\_1 w) P \\).
    
    6.  The issuer selects randomness
        \\( r \xleftarrow{\\$} \mathbb F\_p \\)
        to compute
        \\[
        \operatorname{Enc}\_D(Q) \gets (rB, Q\_c + rD) + b x\_2 \operatorname{Enc}\_D(nB).
        \\]
    
    7.  The issuer forms the proof
        \\[
        \begin{aligned}
        \pi &\gets \operatorname{PK}\\{ \\\\
            &\mathtt{WalletIssuance\\_Issuer}, \\\\
            &(
                b, r, \mathbf x, \widetilde x\_0, t\_2
            ), \\\\
            &(
                P,
                (wP),
                D, 
                \operatorname{Enc}\_D(nB),
                \operatorname{Enc}\_D(Q),
                T\_2
            ), \\\\
            &(\mathbf X, B, \widetilde B) \\; : \\\\
            & X\_0 = x\_0 B + \widetilde x\_0 \widetilde B,  \\\\
            & X\_1 = x\_1 \widetilde B, \\\\
            & X\_2 = x\_2 \widetilde B, \\\\
            & P = bB, \\\\
            & T\_2 = bX\_2, \\\\
            & T\_2 = t\_2 \widetilde B, \\\\
            & \operatorname{Enc}\_D(Q) = 
                (rB, x\_0 P + x\_1 (wP) + rD) + t\_2 \operatorname{Enc}\_D(nB) \\\\
        \\}.
        \end{aligned}
        \\]
        This proof should be added to the transcript from step (2.3), chaining
        the issuer's proof onto the client's proof.  
        
    8.  The issuer sends \\(P\\), \\(\operatorname{Enc}\_D(Q)\\), \\(T\_2\\), and
        \\(\pi\\) to the client.
    
3. **Client**.  The client processes the response as follows:

    1.  The client uses the transcript state from step (1.5) to verify \\(\pi\\).

    2.  The client decrypts \\(Q\\) by computing
        \\[
        Q \gets \operatorname{Enc}\_D(Q)\_1 - d \operatorname{Enc}\_D(Q)\_0.
        \\]
