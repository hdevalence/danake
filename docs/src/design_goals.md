# Design Goals

As described in the introduction, Danake is a lightweight micropayment
system.  A service provider can issue usage credits to users, who can
spend credits for services.  Credit issuance can be anonymous, linked to
a long-term identity, or somewhere in-between, but credits are spent
anonymously, and every credential spend should be unlinkable from all
other credential spends and issuances.

Users should not be able to spend usage credit multiple times or gain
usage credits except from the application's issuance policy.  Danake
does not attempt to prevent users from trading usage credits on a
secondary market, as doing so in the limit is not feasible anyways[^1].
However, Danake also does not attempt to have any independent
verifiability of credit amounts.  Like a gift card, users must trust the
service provider to redeem credits for services.  This model can be
contrasted to the model used in other "layer 2" payment systems, where
counterparty trust is not required.

Danake's micropayment mechanism can either be surfaced to users, or kept
as an implementation detail.  For instance, a service wishing to prevent
overuse by a single user but without wishing to monitor all users and
keep usage statistics could periodically issue a fixed usage quota to
each registered user; their user-agent can automatically spend credits
to use the service as an implementation detail, without ever displaying
a monetary usage quota to the user.  For this reason, this document uses
*user* to refer to a logical user (human or otherwise), and *client* to
refer to a user's user-agent (software operating on their behalf).

The transport between client and server is considered out-of-scope for
Danake, or more precisely, Danake assumes an anonymous transport from
the client to the server.  This can be provided by Tor[^2], or,
alternatively, Danake can be used with HTTPS and an assumption that the
server does not keep logs of user activity.  The latter alternative is
obviously not ideal, but still meaningfully better than having the
service provider track per-user usage statistics.

Finally, because resource controls that cost more than the resources
they are intended to control are impractical, Danake is intended to be
fast enough to replace non-anonymous application authentication logic.

[^1]: For instance, credits could be bound to a specific user-id and the
client could prove consistency between a different user credential and
their wallet credential ... but then users can just make requests on
behalf of each other.

[^2]: To be precise, this requires using a new Tor circuit for each
micropayment the client intends to be unlinkable from previous
micropayments.

