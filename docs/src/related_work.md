# (WIP) Related Work

This page contains an incomplete list of related work. In the future, it will
be expanded to have more detailed comparisons.

## Privacy Pass

[Privacy Pass][pp] is a privacy-preserving authentication protocol developed
by George Tankersley, Filippo Valsorda, Alex Davidson, Ian Goldberg, and Nick
Sullivan. Privacy Pass allows users to request single-use "tokens" from a
server and redeem them unlinkably from their issuance. For instance, a user
can obtain a collection of single-use tokens by solving a CAPTCHA, and later
present those tokens to access web resources. This [page][pp_protocol]
contains a description of the protocol and various attack vectors.

In comparison to Danake tokens, which store a balance that can be drawn down
in multiple spend transactions of different amounts, Privacy Pass tokens can
be redeemed only once for an indivisible unit value. However, Danake's
flexibility comes at a cost, as Privacy Pass redemptions are more efficient
for the server.

## OpenPrivacy's Privacy Pass Extensions

Erinn Atwater and Sarah Jamie Lewis of [OpenPrivacy] are developing
[extensions] to Privacy Pass which allow tokens to be purchased anonymously
using cryptocurrency (e.g., shielded Zcash). These tokens can then be
unlinkably redeemed for anonymous servers. This is the same use-case and
threat model as Danake (TODO: check), but with more efficient token redemption.

## Brave's Privacy Pass Extensions

[Brave] has [a design][Brave_PP] for using Privacy Pass for
privacy-preserving ad confirmations (authors?). Because Privacy Pass tokens
have unit value, they suggest using different (domain-separated) token
issuers to correspond to each token value denomination.

## BOLT

[BOLT] is a "Layer 2" system which provides privacy-preserving off-chain
payment channels with escrowed funds. Either counterparty can close the
channel and obtain their funds net of whatever payments occurred. This is a
more sophisticated threat model than Danake, where the service provider
issues credit according to arbitrary policy and is trusted to redeem credit.

## Hyphae

[Hyphae] was an unimplemented design by Henry de Valence and Isis Lovecruft
for an anonymous reputation-based distribution network for [Tor
bridges][tor_bridges]. It contains precursor ideas to Danake, in particular
an AMACS-based micropayment system, but without credential epochs, key
rotation, the wallet/token distinction, or Bulletproofs.

## Unlinkable Serial Transactions

[Unlinkable Serial Transactions][ust] was a 1997 proposal by Paul Syverson,
Stuart Stubblebine, and David Goldschlag for anonymous service payments. In
USTs, clients sequentially present certificates to a service provider, and
the server responds to each presentation with blinded issuance of a new
certificate. Although the cryptographic mechanisms are different, this
protocol structure is the one used by Danake and Hyphae.

The serial structure of presentation-issuance means that it is not possible
to make multiple payments in parallel. Danake addresses this by means of
token sub-credentials; a user can purchase multiple tokens and use distinct
tokens in parallel, even though each token spend performs a serial
presentation-issuance flow.

## Signal

Signal's [private group design][signal_group] uses AMACs, with some
extensions to allow attributes which are ElGamal-encrypted group elements.


[signal_group]: https://signal.org/blog/pdfs/signal_private_group_system.pdf
[ust]: https://www.ifca.ai/pub/fc97/m4.pdf
[tor_bridges]: https://2019.www.torproject.org/docs/bridges
[Hyphae]: https://patternsinthevoid.net/hyphae/hyphae.pdf
[Bolt]: https://boltlabs.tech/
[Brave]: https://brave.com
[Brave_PP]: https://github.com/brave/brave-browser/wiki/Security-and-privacy-model-for-ad-confirmations
[pp]: https://privacypass.github.io/
[pp_protocol]: https://github.com/privacypass/challenge-bypass-extension/blob/master/docs/PROTOCOL.md
[OpenPrivacy]: https://openprivacy.ca
[OpenPrivacy_Token]: https://git.openprivacy.ca/openprivacy/zcashtokenservice/raw/commit/eacc593342762f0a5364d02f7c22accfdc509789/paper/towards-anonymous-prepaid-services.pdf