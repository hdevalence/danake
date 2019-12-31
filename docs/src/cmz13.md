# CMZ13 Credentials

In their 2013 paper, [_Algebraic MACs and Keyed-Verification Anonymous
Credentials_][amacs], Chase, Meiklejohn, and Zaverucha introduced the
concept of a _keyed-verification anonymous credential_ (KVAC), where the
issuer of an anonymous credential is also the verifier of the
credential, and then showed how to construct KVACs from algebraic
message authentication codes (MACs).

The insight of keyed-verification credentials is that while most
anonymous credential systems are built to allow a user to present
credentials to third parties, in many cases, particularly resource
access control, it's sufficient for verification to be restricted to the
issuer of the credential.  This functional restriction allows much
more efficient constructions, because the credential can use
symmetric-style rather than public-key-style cryptography.

This section reviews the CMZ'13 construction, first giving intuition for
how its components fit together, then providing explicit descriptions of
blinded issuance (which is only sketched in the paper) and credential
presentation.

[amacs]: https://eprint.iacr.org/2013/516
