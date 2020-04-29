# Danake

Danake is a lightweight micropayment system which allows anonymous
services to bill for resource usage without compromising privacy or
performance.  It is named in reference to the [danake], a silver coin of
the Persian empire commonly placed in the mouth of the deceased to pay
for passage into the underworld.

Using Danake, a service provider can issue credits to users based on some
application-dependent policy, and allow those users to anonymously spend
those credits to pay for services.  The exact policy is considered
out-of-scope for Danake itself, but for instance, a service provider
could periodically issue credits to each user, or allow users to
purchase credits using Zcash, etc.

Danake is a work-in-progress. The rendered description of the scheme can be
found at https://lightweight.money .

A brief overview of the ideas behind Danake can be found in [this
thread](https://twitter.com/hdevalence/status/1253155347327356928).

### Tests

Run `cargo test`.

### Docs

Run `cargo doc --no-deps --open`, or omit `--no-deps` to include docs for all
dependent crates, or omit `--open` to skip opening the browser.

### Website

Inside the `docs` directory, run `mdbook build` to build the site or `mdbook
serve` to preview changes locally. Run `firebase deploy` to deploy the site.

[danake]: https://en.wikipedia.org/wiki/Danake