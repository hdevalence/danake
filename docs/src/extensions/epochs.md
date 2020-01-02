# Credential Epochs

One practical issue with CMZ13 credentials is that the issuer's secret
key material must be kept online to verify previously-issued
credentials, and it must outlive the lifetime of the longest-lived
credential.  The first factor increases the risk of key compromise,
while the second factor means that unplanned key rotation is effectively
impossible, as it requires coordination from all credential holders.

For this reason, it's important to integrate a notion of key rotation
into the system from the beginning:

> if you're _always_ rotating credentials, then credential rotation is
> just another thing your system does -- it's not a special mode that
> you have to reason about.
> [@endsofthreads](https://twitter.com/endsofthreads/status/1212440743958253570)

To do this, we treat each credential type \\(T\\) as being a family of
dependent types indexed by integers.  Integers correspond to time
intervals called *epochs*, and each type \\(T_i\\) represents a
credential of type \\(T\\) in epoch \\(i\\).  Each CMZ13 credential is
issued with respect to some issuance parameters, and each epoch \\(i\\) has
designated _primary_ parameters for credentials of type \\(T\\).

Next, we define a _rollover_ procedure which allows a client to convert
a credential of type \\(T_i\\) into a credential of type \\(T\_{i+1}\\).
To do this, the client simultaneously presents a credential of type \\(T\_i\\)
and requests blinded issuance of a new credential of type
\\(T\_{i+1}\\), while supplying a zero-knowledge proof that the
requested attributes are identical to the previous credential's
attributes.  If the previous credential had a nullifier, the client
reveals the nullifier of the previous credential and generates a new
nullifier for the new credential.

However, for the issuer to accept the previous credential or to issue a
new credential, the issuer must have issuance parameters for both epochs
\\(i\\) and \\(i+1\\).[^1]  And because presentations are prepared by the
client, the client should ideally be able to determine an epoch to use
for a presentation without having to agree on a clock.  To address both
of these issues, we adopt a key schedule loosely inspired by Google's
Cloud KMS.  In this schedule, issuance parameters can have the following
states:

* **Primary**, meaning that these issuance parameters are the canonical
  issuance parameters for this epoch, and can be used for credential
  presentations and rollovers;
* **Active**, meaning that these issuance parameters are not the
  canonical issuance parameters for this epoch, but can still be used
  for credential presentations and rollovers;
* **Rollover**, meaning that these issuance parameters cannot be used
  for credential presentations except for rollovers.

Each set of issuance parameters passes through states Active, Primary,
Active, and Rollover before deletion, as indicated in the following
diagram:

```
        ─────────Epoch─────────────────▶
        -3 -2 -1  0  1  2  3  4  5  6  7

    │    A  P  A  R
    │       A  P  A  R
    │          A  P  A  R
    │             A  P  A  R
 Issuance            A  P  A  R
Parameters              A  P  A  R
    │                      A  P  A  R
    ▼                         A  P  A  R
```

Because the issuer accepts credentials issued with respect to the
primary parameters from both previous and subsequent epochs, the client
and issuer do not need to agree on exactly when the epoch boundary
occurs.  By observing late use of old parameters or early use of new
parameters respectively, the issuer can detect when a client's clock is
slightly slower or slightly faster than the issuer's clock, but only for
a limited time and a specific request, because either (in the slower
case) the client will soon roll over their credential or (in the faster
case) other clients will soon roll over their credentials.

Because the issuer parameters are declared in advance (they become
active parameters before they become primary parameters), clients can
fetch parameters in advance, either from the issuer or from a
transparency log.  This forces the issuer to commit to specific
parameters and prevents key-partitioning attacks.

This design requires that every client is online at least once every
four epochs.  The *epoch duration* is a deployment-specific system
parameter that should be chosen so that this assumption is reasonable.
To make the epoch index easy to calculate, the reference epoch is set to
start at UNIX timestamp `0`, so that the current epoch index can be
calculated by dividing a UNIX timestamp by the epoch duration in
seconds.

When using credentials with nullifiers, as described in the previous
section, each credential type \\(T_i\\) has a distinct nullifier set,
and this set must be preserved as long as the corresponding parameters
are in Active, Primary, or Rollover states.  However, as soon as the
parameters expire, the nullifier set can be immediately deleted.  In
this way, the epoch system for key rotation also automatically prunes
the nullifier sets for each credential.  Moreover, although the
nullifier sets for parameters in Active and Rollover states need to be
retained, they are likely to be accessed less frequently, so that only
the nullifier set for the Primary parameters is in the hot path.

[^1]: In practice it makes more sense to define a map from \\(T\_i\\) to
\\(T\_{i+k}\\) to allow \\(k\\)-step rollover in case a client was
offline for more than one epoch.
