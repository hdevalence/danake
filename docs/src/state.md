# The Danake State Machine

Danake allows a service provider to issue credits to users based on some
application-dependent policy.  The exact policy is considered
out-of-scope for Danake itself, but for instance, a service provider
could periodically issue a fixed number of credits to each user, or
allow users to purchase credits using Zcash, etc.  

A user's credit balance is stored in a *wallet* credential with
attributes \\((w,n_w)\\), where \\(w\\) is the wallet balance and
\\(n_w\\) is the wallet nullifier.  Clients can transfer small portions of
their wallet balance to *tokens*, which can be used to spend credits
with the service provider until the token balance is depleted.  A token
is a credential with attributes \\((t,n_t)\\), where \\(t\\) is the
token balance and \\(n_t\\) is the token nullifier.  Both wallets and
tokens have nullifier attributes so that they can only be used once;
each presentation of a wallet or token is coupled with a (blinded)
issuance request for a new wallet or token with an updated balance and a
new nullifier.

While the simplest mechanism would be for the client to spend credits from
the wallet credential directly, using intermediate tokens has a number
of advantages:

1.  Because presentation is sequential (each presentation reveals a
nullifier and requests issuance of a new credential for future use),
using intermediate tokens allows a client to make multiple presentations
in parallel, rather than waiting for a complete server round-trip before
spending more credits.

2.  Because token credentials store less value than the wallet
credentials, they can use smaller bitsizes for their range proofs,
reducing the server's verification work for the more commonly used
credentials.

3.  Because token credentials have different issuer parameters, the
epoch interval for token credentials can be shorter than the epoch
interval for wallet credentials, which allows faster pruning of the
nullifier sets for the more commonly used credentials.

4.  Because token credentials have different issuer parameters, a
deployment in a sharded setting (e.g., to a service backed by a CDN)
could scope tokens to a particular shard (e.g., a CDN edge region or
point-of-presence), requiring only local rather than global nullifier
state for the more commonly used credentials.

The remainder of this chapter (table of contents to the left) describes
the state transitions in Danake.

An example sequence of state transitions is illustrated below.
Rectangular boxes denote credential states, and round boxes denote state
changes.  Each state change requires exactly one round-trip from the
client to the server.  Note that the value `v` in each spend step is not
hardcoded but can be any arbitrary policy-determined credit amount.

```ascii
 .───────────────────────.
(Wallet issuance (policy) )
 `───────────────────────'
             │
             ▼
      ┌─────────────┐
      │   Wallet    │   w_0:  wallet balance
      │ (w_0, nw_0) │   nw_0: wallet nullifier
      └─────────────┘
             │
             ▼                                  ┌─────────────┐
   .───────────────────.                        │    Token    │
  (Buy token, value t_0 )──────────────────────▶│ (t_0, nt_0) │
   `───────────────────'                        └─────────────┘
             │                                         │
             ▼                                         ▼
     ┌───────────────┐                           .───────────.
     │    Wallet     │                          (Spend value v)
     │  (w_1, nw_1)  │                           `───────────'
     │w_1 = w_0 - t_0│                                 │
     └───────────────┘                                 ▼
             │                                  ┌─────────────┐
             ▼               ┌─────────────┐    │    Token    │
   .───────────────────.     │    Token    │    │ (t_1, nt_1) │
  (Buy token, value t_0 )───▶│ (t_0, nt_0) │    │t_1 = t_0 - v│
   `───────────────────'     └─────────────┘    └─────────────┘
             │                      │                  │
             ▼                      ▼                  ▼
     ┌───────────────┐        .───────────.      .───────────.
     │    Wallet     │       (Spend value v)    (Spend value v)
     │  (w_1, nw_1)  │        `───────────'      `───────────'
     │w_1 = w_0 - t_0│              │                  │
     └───────────────┘              ▼                  ▼
             │               ┌─────────────┐    ┌─────────────┐
             ▼               │    Token    │    │    Token    │
   .───────────────────.     │ (t_1, nt_1) │    │ (t_2, nt_2) │
  (Add value v (policy) )    │t_1 = t_0 - v│    │t_2 = t_1 - v│
   `───────────────────'     └─────────────┘    └─────────────┘
             │                      │                  │
             ▼                      ▼
     ┌───────────────┐        .───────────.            │
     │    Wallet     │       (Spend value v)
     │  (w_2, nw_2)  │        `───────────'            │
     │ w_2 = w_1 + v │              │
     └───────────────┘              ▼                  ▼
             │               ┌─────────────┐    ┌─────────────┐
                             │    Token    │    │    Token    │
             │               │ (t_2, nt_2) │    │ (t_i, nt_i) │
                             │t_2 = t_1 - v│    │   t_i < v   │
             │               └─────────────┘    └─────────────┘
                                    │                  │ Drop
             ▼                                         │change
                                    │                  ▼
                                                       Λ
                                    │                 ▕ ▏
                                                       V
                                    ▼
                             ┌─────────────┐
                             │    Token    │
                             │ (t_i, nt_i) │
                             │   t_i < v   │
                             └─────────────┘
                                    │ Drop
                                    │change
                                    ▼
                                    Λ
                                   ▕ ▏
                                    V
```
