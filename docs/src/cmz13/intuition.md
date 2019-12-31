# Intuition

A message authentication code (MAC) scheme allows the holder of a secret
key to produce a MAC witnessing the integrity of the message, which they
can verify using the same secret key.  This is the
symmetric-cryptagraphy analogue of a public-key signature scheme, in
which the holder of a secret key can produce a signature witnessing the
integrity of the message, which anyone can verify using the
corresponding public key.

MACs allow the construction of keyed-verification credential systems,
which the issuer of a credential uses a MAC to witness the integrity of
a credential and later checks it on presentation.  Most famously, this
is the idea underlying Macaroons, which support other interesting
features such as delegation and attenuation.

CMZ13 take this idea further, showing how to construct anonymous 
