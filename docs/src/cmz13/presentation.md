# Presentation

FIXME: change description to explain why this works

Credential presentation is an online protocol with the following steps. As in
the description of issuance, the the set of hidden attribute indexes is
denoted by \\(\mathcal H \subseteq \\{1, \ldots, n\\}\\). However, the set
\\(\\mathcal H\\) need not be the same – different sets of attributes can be
hidden or revealed between issuance and presentation.

1. **Client**.  Given attributes \\(\mathbf m\\) and a previously issued tag 
    \\(P\_0, Q\_0\\), the client proceeds as follows.
    1.  The client re-randomizes the tag by choosing
        \\(r \xleftarrow{\\$} \mathbb F\_p\\) and computing
        \\((P, Q) \gets (r P\_0, r Q\_0)\\).
    2.  The client commits to the hidden attributes by choosing
        \\(
        \widetilde m\_i \xleftarrow{\\$} \mathbb F\_p
        \\)
        and computing
        \\(
        \operatorname{Com}(m\_i) = m\_i P + \widetilde m\_i \widetilde B
        \\)
        for each \\(i \in \mathcal H\\).
        Notice that the Pedersen commitments are made with 
        respect to the Pedersen generators \\((P, \widetilde B)\\)
        rather than \\((B, \widetilde B)\\).
    3.  The client commits to \\(Q\\) by choosing
        \\(
        \widetilde r \xleftarrow{\\$} \mathbb F\_p
        \\)
        and computing
        \\(
        C\_Q = Q + rB
        \\).
    3.  The client uses the issuance parameters to compute a correction term
        \\(
            V \gets \sum\_{i \in \mathcal H} \widetilde m\_i - rB
        \\).
    4.  The client proves that the commitments and the correction term were computed correctly:
       \\[
       \begin{aligned}
       \pi &\gets \operatorname{PK}\\{ \\\\
           &\mathtt{ClientPresentation}, \\\\
           &(r, (m\_i, \widetilde m\_i)\_{i \in \mathcal H}), \\\\
           &(P, (\operatorname{Com}(m\_i))\_{i \in \mathcal H}), \\\\
           &(B, \widetilde B) \\; : \\\\
           & \operatorname{Com}(m\_i) = m\_i P + \widetilde m\_i \widetilde B \quad \forall i \in \mathcal H \\\\
           & V = \sum\_{i \in \mathcal H} \widetilde m\_i X\_i - rB \\\\
       \\}
       \end{aligned}
       \\]
    5.  The client sends \\(P\\), \\(C\_Q\\), 
        \\((\operatorname{Com}(m\_i))\_{i \in \mathcal H}\\),
        \\((m\_i)\_{i \not\in \mathcal H}\\), and \\(\pi\\) to the issuer.
2. **Issuer**.  The issuer computes \\(V\\) as
    \\[
        V \gets 
        \Big(
            x\_0 + \sum\_{i \in \mathcal H} x\_i m\_i
        \Big) P +
        \sum\_{i \in \mathcal H} x\_i \operatorname{Com}(m\_i)\_{i \in \mathcal H} -
        C\_Q
    \\]
    and uses \\(V\\) to verify \\(\pi\\).