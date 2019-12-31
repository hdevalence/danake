# CMZ13 Credentials

A message authentication code (MAC) scheme allows the holder of a secret
key to produce a MAC witnessing the integrity of the message, which they
can verify using the same secret key.  This is the
symmetric-cryptography analogue of a public-key signature scheme, in
which the holder of a secret key can produce a signature witnessing the
integrity of the message, which anyone can verify using the
corresponding public key.

MACs allow the construction of keyed-verification credential systems,
which the issuer of a credential uses a MAC to witness the integrity of
a credential and later checks it on presentation.  Most famously, this
is the idea underlying Macaroons, which support other interesting
features such as delegation and attenuation.

In their 2013 paper, [_Algebraic MACs and Keyed-Verification Anonymous
Credentials_][amacs], Chase, Meiklejohn, and Zaverucha apply this idea
to anonymous credentials, introducing the 
concept of a keyed-verification anonymous credential (KVAC), where the
issuer of an anonymous credential is also the verifier of the
credential, and then showed how to construct anonymous credentials using
*algebraic* MACs, i.e., MACs defined in some group rather than using
bitwise operations.

The insight of keyed-verification credentials is that while most
anonymous credential systems are built to allow a user to present
credentials to third parties, in many cases, particularly resource
access control, it's sufficient for verification to be restricted to the
issuer of the credential.  This functional restriction allows much
more efficient constructions, because the credential can use
symmetric-style rather than public-key-style cryptography.

This section reviews the CMZ'13 construction, describing key generation,
blinded issuance, and credential presentation.

[amacs]: https://eprint.iacr.org/2013/516
