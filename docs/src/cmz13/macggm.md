# MAC-GGM

The CMZ13 paper defines two algebraic MACs which can be used to
construct credentials, \\(\\mathsf{MAC\_{GGM}}\\) and
\\(\\mathsf{MAC\_{DDH}}\\), which are proved secure in the generic group
model and under the decisional Diffie-Hellman assumption, respectively.
However, \\(\\mathsf{MAC\_{DDH}}\\) is more expensive and as noted in
the paper, there is no reason to believe that \\(\\mathsf{MAC\_{GGM}}\\)
is not also secure under DDH, although there is no proof.  

For reasons of efficiency we use \\(\\mathsf{MAC\_{GGM}}\\).  As noted
in the paper, this is a generalization of a [2012
construction][mac_revisited] due to Yevgeniy Dodis, Eike Kiltz,
Krzysztof Pietrzak, and Daniel Wichs.  It is defined as follows:

* **Setup**.  Choose \\(\mathbb G\\) a group of prime order \\(p\\).
  The messages will be elements of \\(\\mathbb
  F\_p^n\\), i.e., \\(n\\)-tuples of field elements in \\(\\mathbb F_p\\).

* **Key Generation**. Choose
  \\( \mathbf x = (x\_0, x\_1, \ldots, x\_n) \xleftarrow{\\$} \mathbb F\_p^{n+1}
  \\).

* **MAC**.  Given a message \\(\mathbf m \in \mathbb F\_p^n\\), choose
  \\(P \xleftarrow{\\$} \mathbb G \setminus \\{ 0 \\} \\) and compute 
  \\(Q = \langle \mathbf x, (1) || \mathbf m \rangle \cdot P\\).  The
  tag is \\((P, Q)\\).
 
* **Verify**.  Given a message \\(\mathbf m \in \mathbb F\_p^n\\) and a
  purported tag \\( (P,Q)\\), accept if \\(P \neq 0\\) and
  \\(Q = \langle \mathbf x, (1) || \mathbf m \rangle \cdot P\\),
  otherwise reject.

[mac_revisited]: https://eprint.iacr.org/2012/059

