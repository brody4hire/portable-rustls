# Security Policy

<!-- TODO(portable-rustls) UPDATE INFO BELOW FOR THIS FORK -->

__IMPORTANT NOTICE:__ regardless of upstream __`rustls`__ project this fork is __NOT CERTIFIED__ and __NOT PEER-REVIEWED__ - USE AT YOUR OWN RISK (as stated elsewhere)

## Supported Versions

<!-- TODO(portable-rustls) UPDATE INFO FOR THIS FORK -->
TODO

<!-- TODO(portable-rustls) UPDATE INFO FOR THIS FORK:
Security fixes will be backported only to the rustls versions for which the
original semver-compatible release was published less than 2 years ago.

For example, as of 2024-04-18 the latest release is 0.23.4.

* 0.23.0 was released in February of 2024
* 0.22.0 was released in December of 2023
* 0.21.0 was released in March of 2023
* 0.20.0 was released in September of 2021
* 0.19.0 was released in November of 2020

Therefore 0.23.x, 0.22.x and 0.21.x will be updated, while 0.20.x and 0.19.x
will not be.

> [!NOTE]
> We use the date of `crates.io` publication when evaluating the security
> policy. For example, while the Rustls 0.20.0 GitHub release note was created
> Jul, 2023 the actual release in `crates.io` was published in Sept. 2021.
-- -->

### Minimum Supported Rust Version

<!-- TODO(portable-rustls) UPDATE INFO FOR THIS FORK -->
TODO

<!-- TODO(portable-rustls) UPDATE INFO FOR THIS FORK:
From time to time we will update our minimum supported Rust version (MSRV)
in the course of normal development, subject to these constraints:

- Our MSRV will be no more recent than 9 versions old, or approximately 12 months.

> [!TIP]
> At the time of writing, the most recent Rust release is 1.82.0.  That means
> our MSRV could be as recent as 1.73.0. As it happens, it is 1.71.0.

- Our MSRV policy only covers the core library crate: it does not cover tests
  or example code, and is not binding on our dependencies.

- We do not consider MSRV changes to be breaking for the purposes of semver.

- Once we reach 1.0.0, we will not make MSRV changes in patch releases.
  (Prior to reaching 1.0.0, cargo does not support patch releases.)

- We will not make MSRV changes to security maintenance branches.

> [!NOTE]
> For the avoidance of doubt: security maintenance branches exist for each
> release line _that is not the latest_: so (at the time of writing) 0.22
> and 0.21 have maintenance branches, but 0.23 does not and is released from
> the `main` branch.
-- -->

#### MSRV of new dependencies

We may take _non-default_ optional new dependencies on a crate with a later
MSRV than this policy.

> [!NOTE]
> This is currently the case for our optional dependency on `zlib-rs`, which
> has a current MSRV of 1.75.0.

## Reporting a Vulnerability

<!-- TODO(portable-rustls) CLEANUP & UPDATE WITH MORE SPECIFIC INFO FOR THIS FORK: -->
PLEASE REPORT ANY POSSIBLE OR POTENTIAL SECURITY ISSUES SPECIFIC TO THIS FORK PRIVATELY TO A MAINTAINER OF THIS FORK or [open a draft security advisory on `portable-rustls` in GitHub](https://github.com/brody4hire/portable-rustls/security/advisories/new), NOT VIA PUBLIC GITHUB ISSUE, DISCUSSION or PR.

<!-- TODO(portable-rustls) CLEANUP & UPDATE WITH MORE SPECIFIC INFO: -->
FOR ANY POSSIBLE OR POTENTIAL SECURITY ISSUES WITH __`rustls`__ IN GENERAL:
Please report security bugs [privately to `rustls` via github](https://github.com/rustls/rustls/security/advisories/new).
AS STATED ABOVE: NOT VIA PUBLIC GITHUB ISSUE, DISCUSSION or PR.

<!-- TODO(portable-rustls) UPDATE WITH MORE SPECIFIC INFO FOR THIS FORK: -->
We'll then:

<!-- TODO(portable-rustls) UPDATE INFO FOR THIS FORK: -->
- Prepare a fix and regression tests.
- Backport the fix and make a patch release for most recent release.
- Submit an advisory to [rustsec/advisory-db](https://github.com/RustSec/advisory-db).
- Refer to the advisory on the main README.md and release notes.

If you're *looking* for security bugs, this crate is set up for
`cargo fuzz` but would benefit from more runtime, targets and corpora.
