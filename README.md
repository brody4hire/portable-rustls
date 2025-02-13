<!-- TODO(portable-rustls) UPDATE FOR THIS FORK:
<p align="center">
  <img width="460" height="300" src="https://raw.githubusercontent.com/rustls/rustls/main/admin/rustls-logo-web.png">
</p>
-- -->

<!-- SUPERSEDED IN THIS FORK
<p align="center">
Rustls is a modern TLS library written in Rust.
</p>
-->

<!-- TODO(portable-rustls) UPDATE INFO & LINKS BELOW FOR THIS FORK -->

# portable-rustls - a fork of upstream rustls

<p align="center">☆ ☆ ☆ (a modern TLS library IMPLEMENTED IN RUST) ☆ ☆ ☆</p>

[![Crates.io Version](https://img.shields.io/crates/v/portable-rustls?style=flat-square)](https://crates.io/crates/portable-rustls)
[![docs.rs](https://img.shields.io/docsrs/portable-rustls?style=flat-square&label=docs.rs)](https://docs.rs/portable-rustls/)
![Crates.io MSRV](https://img.shields.io/crates/msrv/portable-rustls?style=flat-square)

<!-- TODO(portable-rustls) CLEANUP & IMPROVE NOTE FOR THIS FORK; REWORK TO AVOID REPEATED INFO -->
IMPORTANT NOTICE: REGARDLESS OF UPSTREAM `rustls` PROJECT THIS FORK IS NOT CERTIFIED AND NOT PEER-REVIEWED - USE AT YOUR OWN RISK (AS STATED FURTHER BELOW)

<!-- TODO(portable-rustls) CLEANUP & IMPROVE NOTE FOR THIS FORK; REWORK TO AVOID REPEATED INFO -->
RECOMMENDED USAGE OF THIS FORK (as stated further below):
* USE DEPENDENCY LIKE THIS IN `Cargo.toml`: `rustls = { package = "portable-rustls", ... }`
* NEED TO EXPLICITLY ENABLE ANY FEATURES AS NEEDED - NO FEATURES ARE ENABLED BY DEFAULT IN THIS FORK
* IMPORT AS USUAL FROM `rustls`: `use rustls;` OR `use rustls::...`

THIS FORK SUPPORTS BUILDING FOR TARGETS WITH NO ATOMIC PTR - SEE INFO FURTHER BELOW

<!-- XXX TODO ADD NOTE THAT ARC IS NOW ALWAYS USED FROM XXX XXX AS XXX TO BE STATED IN XXX - XXX TODO NEEDS TO BE STATED FURTHER BELOW & IN DOC FROM lib.rs  -->
ADDITIONAL NOTE (as stated further below): FIPS SUPPORT IS REMOVED FROM THIS FORK. THERE MAY BE SOME VESTIGES IN THE API,
IMPLEMENTATION OR DOCUMENTATION BUT THIS DOES NOT IMPLY EXISTENCE OF FIPS SUPPORT IN ANY FORM.

# Status

<!-- TODO(portable-rustls) UPDATE & CLEAN UP INFO FOR THIS FORK -->
FROM UPSTREAM RUSTLS:
Rustls is used in production at many organizations and projects. We aim to maintain
reasonable API surface stability but the API may evolve as we make changes to accommodate
new features or performance improvements.

We have a [roadmap](ROADMAP.md) for our future plans. We also have [benchmarks](BENCHMARKING.md) to
prevent performance regressions and to let you evaluate rustls on your target hardware.

If you'd like to help out, please see [CONTRIBUTING.md](CONTRIBUTING.md).

<!-- TODO(portable-rustls) INCLUDE MORE OF THIS INFO IN BADGES FURTHER ABOVE FOR THIS FORK (if possible):)
[![Build Status](https://github.com/rustls/rustls/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/rustls/rustls/actions/workflows/build.yml?query=branch%3Amain)
[![Coverage Status (codecov.io)](https://codecov.io/gh/rustls/rustls/branch/main/graph/badge.svg)](https://codecov.io/gh/rustls/rustls/)
[![Documentation](https://docs.rs/rustls/badge.svg)](https://docs.rs/rustls/)
[![Chat](https://img.shields.io/discord/976380008299917365?logo=discord)](https://discord.gg/MCSB76RU96)
[![OpenSSF Best Practices](https://www.bestpractices.dev/projects/9034/badge)](https://www.bestpractices.dev/projects/9034)
-- -->

## Changelog

The detailed list of changes in each release can be found in: https://github.com/brody4hire/portable-rustls/releases

# Documentation

https://docs.rs/portable-rustls/

# Approach

<!-- TODO(portable-rustls) CLEANUP & IMPROVE NOTE FOR THIS FORK -->
IMPORTANT NOTICE: REGARDLESS OF UPSTREAM `rustls` PROJECT THIS FORK IS NOT CERTIFIED AND NOT PEER-REVIEWED - USE AT YOUR OWN RISK AS STATED FURTHER BELOW

<!-- TODO(portable-rustls) CLEANUP & IMPROVE NOTE FOR THIS FORK -->
RECOMMENDED USAGE OF THIS FORK:
* USE DEPENDENCY LIKE THIS IN `Cargo.toml`: `rustls = { package = "portable-rustls", ... }`
* NEED TO EXPLICITLY ENABLE ANY FEATURES AS NEEDED - NO FEATURES ARE ENABLED BY DEFAULT IN THIS FORK
* IMPORT AS USUAL FROM `rustls`: `use rustls;` OR `use rustls::...`

<!-- TODO(portable-rustls) CLEANUP & IMPROVE DOCUMENTATION OF THIS FEATURE -->
THIS FORK SUPPORTS using `Arc` from `portable-atomic-util` to support targets with no atomic ptr, with the following requirements:
* USE Rust nightly toolchain
* USE `--cfg portable_atomic_unstable_coerce_unsized` in RUSTFLAGS FOR `cargo build` (etc.)
* ENABLE `critical-section` feature of this crate

<!-- TODO: ADDRESS HOW TO BUILD WITH A CRYPTO PROVIDER ON A TARGET WITH NO ATOMIC PTR -->
<!-- (MAYBE BUILD WITH A BUILT-IN CRYPTO PROVIDER OR MAYBE THIRD-PARTY CRYPTO PROVIDER) -->
ALSO NEED TO BUILD WITH A CRYPTO PROVIDER FOR THIS CRATE TO BE USEFUL IN GENERAL.

ADDITIONAL NOTE: FIPS SUPPORT IS REMOVED FROM THIS FORK. THERE MAY BE SOME VESTIGES IN THE API,
IMPLEMENTATION OR DOCUMENTATION BUT THIS DOES NOT IMPLY EXISTENCE OF FIPS SUPPORT IN ANY FORM.

<!-- TODO(portable-rustls) UPDATE INFO FOR THIS FORK -->
Rustls is a TLS library that aims to provide a good level of cryptographic security,
requires no configuration to achieve that security, and provides no unsafe features or
obsolete cryptography by default.

<!-- TODO(portable-rustls) UPDATE INFO FOR THIS FORK -->
Rustls implements TLS1.2 and TLS1.3 for both clients and servers. See [the full
list of protocol features](https://docs.rs/rustls/latest/rustls/manual/_04_features/index.html).

### Platform support

While Rustls itself is platform independent, there are some additional platform requirements
for the built-in providers.

[`aws-lc-rs`] is commonly used as the provider that implements the cryptography in TLS.
See [the aws-lc-rs FAQ][aws-lc-rs-platforms-faq] for more details of the
platform/architecture support constraints in aws-lc-rs.

[`ring`] is also available via the `ring` crate feature: see
[the supported `ring` target platforms][ring-target-platforms].

By providing a custom instance of the [`crypto::CryptoProvider`] struct, you
can replace all cryptography dependencies of rustls.  This is a route to being portable
to a wider set of architectures and environments, or compliance requirements.  See the
[`crypto::CryptoProvider`] documentation for more details.

Specifying `default-features = false` when depending on rustls will remove the
dependency on aws-lc-rs.

Rustls requires Rust 1.63 or later. It has an optional dependency on zlib-rs which requires 1.75 or later.

[ring-target-platforms]: https://github.com/briansmith/ring/blob/2e8363b433fa3b3962c877d9ed2e9145612f3160/include/ring-core/target.h#L18-L64
[`crypto::CryptoProvider`]: https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html
[`ring`]: https://crates.io/crates/ring
[aws-lc-rs-platforms-faq]: https://aws.github.io/aws-lc-rs/faq.html#can-i-run-aws-lc-rs-on-x-platform-or-architecture
[`aws-lc-rs`]: https://crates.io/crates/aws-lc-rs

### Cryptography providers

Since Rustls 0.22 it has been possible to choose the provider of the cryptographic primitives
that Rustls uses. This may be appealing if you have specific platform, compliance or feature
requirements that aren't met by the default provider, [`aws-lc-rs`].

Users that wish to customize the provider in use can do so when constructing `ClientConfig`
and `ServerConfig` instances using the `with_crypto_provider` method on the respective config
builder types. See the [`crypto::CryptoProvider`] documentation for more details.

#### Built-in providers

Rustls ships with two built-in providers controlled by associated crate features,
which are both optional in this fork:

* [`aws-lc-rs`] - available with the `aws-lc-rs` crate feature enabled.
* [`ring`] - available with the `ring` crate feature enabled.

See the documentation for [`crypto::CryptoProvider`] for details on how providers are
selected.

#### Third-party providers

<!-- TODO(portable-rustls) CLEANUP THIS NOTICE & IMPROVE THE INFO HERE: -->
NOTICE: ANY THIRD-PARTY PROVIDER WOULD NEED TO BE ADAPTED TO WORK DIRECTLY WITH THIS FORK OF RUSTLS.

The community has also started developing third-party providers for Rustls:

* [`rustls-mbedtls-provider`] - a provider that uses [`mbedtls`] for cryptography.
* [`rustls-openssl`] - a provider that uses [OpenSSL] for cryptography.
* [`boring-rustls-provider`] - a work-in-progress provider that uses [`boringssl`] for
cryptography.
* [`rustls-rustcrypto`] - an experimental provider that uses the crypto primitives
from [`RustCrypto`] for cryptography.
* [`rustls-symcrypt`] - a provider that uses Microsoft's [SymCrypt] library.
* [`rustls-wolfcrypt-provider`] - a work-in-progress provider that uses [`wolfCrypt`] for cryptography.

[`rustls-mbedtls-provider`]: https://github.com/fortanix/rustls-mbedtls-provider
[`mbedtls`]: https://github.com/Mbed-TLS/mbedtls
[`rustls-openssl`]: https://github.com/tofay/rustls-openssl
[OpenSSL]: https://openssl-library.org/
[`rustls-symcrypt`]: https://github.com/microsoft/rustls-symcrypt
[SymCrypt]: https://github.com/microsoft/SymCrypt
[`boring-rustls-provider`]: https://github.com/janrueth/boring-rustls-provider
[`boringssl`]: https://github.com/google/boringssl
[`rustls-rustcrypto`]: https://github.com/RustCrypto/rustls-rustcrypto
[`RustCrypto`]: https://github.com/RustCrypto
[`rustls-wolfcrypt-provider`]: https://github.com/wolfSSL/rustls-wolfcrypt-provider
[`wolfCrypt`]: https://www.wolfssl.com/products/wolfcrypt

#### Custom provider

<!-- TODO(portable-rustls) UPDATE INFO BELOW AS MAY BE NEEDED FOR THIS FORK -->

We also provide a simple example of writing your own provider in the [`custom-provider`]
example. This example implements a minimal provider using parts of the [`RustCrypto`]
ecosystem.

<!-- TODO(portable-rustls) CLEANUP THIS NOTE & IMPROVE THE INFO HERE: -->
HIGHLY RECOMMENDED TO LOOK INTO `provider-example` SUBDIRECTORY ([`provider-example`](provider-example/)) IN THIS FORK; SEE ESPECIALLY `provider-example/Cargo.toml`.
AS DISCUSSED ABOVE, HIGHLY RECOMMENDED TO PUT DEPENDENCY LIKE THIS INTO `Cargo.toml`: `rustls = { package = "portable-rustls" ... }`

See the [Making a custom CryptoProvider] section of the documentation for more information
on this topic.

<!-- TODO(portable-rustls) UPDATE WITH BETTER REFERENCES FOR THIS FORK: -->
[`custom-provider`]: https://github.com/rustls/rustls/tree/main/provider-example/
[`RustCrypto`]: https://github.com/RustCrypto
[Making a custom CryptoProvider]: https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html#making-a-custom-cryptoprovider

# Example code

Our [examples] directory contains demos that show how to handle I/O using the
[`stream::Stream`] helper, as well as more complex asynchronous I/O using [`mio`].
If you're already using Tokio for an async runtime you may prefer to use
[`tokio-rustls`] instead of interacting with rustls directly.

<!-- TODO(portable-rustls) CLEANUP THIS NOTE & IMPROVE THE INFO HERE: -->
NOTE: SOME REFERENCES MAY NEED ADAPTATION TO WORK WITH THIS FORK

<!-- TODO(portable-rustls) CLEANUP THIS NOTE & IMPROVE THE INFO HERE: -->
ADDITIONAL NOTE: `tokio-rustls` WOULD NEED TO BE ADAPTED TO WORK WITH THIS FORK.

The [`mio`] based examples are the most complete, and discussed below. Users
new to Rustls may prefer to look at the simple client/server examples before
diving in to the more complex MIO examples.

[examples]: examples/
[`stream::Stream`]: https://docs.rs/rustls/latest/rustls/struct.Stream.html
[`mio`]: https://docs.rs/mio/latest/mio/
[`tokio-rustls`]: https://docs.rs/tokio-rustls/latest/tokio_rustls/

## Client example program

The MIO client example program is named `tlsclient-mio`.

Some sample runs:

```
$ cargo run --bin tlsclient-mio -- --http mozilla-modern.badssl.com
HTTP/1.1 200 OK
Server: nginx/1.6.2 (Ubuntu)
Date: Wed, 01 Jun 2016 18:44:00 GMT
Content-Type: text/html
Content-Length: 644
(...)
```

or

```
$ cargo run --bin tlsclient-mio -- --http expired.badssl.com
TLS error: InvalidCertificate(Expired)
Connection closed
```

Run `cargo run --bin tlsclient-mio -- --help` for more options.

## Server example program

The MIO server example program is named `tlsserver-mio`.

Here's a sample run; we start a TLS echo server, then connect to it with
`openssl` and `tlsclient-mio`:

```
$ cargo run --bin tlsserver-mio -- --certs test-ca/rsa-2048/end.fullchain --key test-ca/rsa-2048/end.key -p 8443 echo &
$ echo hello world | openssl s_client -ign_eof -quiet -connect localhost:8443
depth=2 CN = ponytown RSA CA
verify error:num=19:self signed certificate in certificate chain
hello world
^C
$ echo hello world | cargo run --bin tlsclient-mio -- --cafile test-ca/rsa-2048/ca.cert --port 8443 localhost
hello world
^C
```

Run `cargo run --bin tlsserver-mio -- --help` for more options.

# License

<!-- TODO(portable-rustls) UPDATE HERE & IN LICENSE FILES AS MAY BE NEEDED FOR THIS FORK -->
Rustls is distributed under the following three licenses:

- Apache License version 2.0.
- MIT license.
- ISC license.

These are included as LICENSE-APACHE, LICENSE-MIT and LICENSE-ISC
respectively.  You may use this software under the terms of any
of these licenses, at your option.

<!-- TODO(portable-rustls) UPDATE INFO FOR THIS FORK:
# Project Membership

- Joe Birr-Pixton ([@ctz], Project Founder - full-time funded by [Prossimo])
- Dirkjan Ochtman ([@djc], Co-maintainer)
- Daniel McCarney ([@cpu], Co-maintainer)
- Josh Aas ([@bdaehlie], Project Management)

[@ctz]: https://github.com/ctz
[@djc]: https://github.com/djc
[@cpu]: https://github.com/cpu
[@bdaehlie]: https://github.com/bdaehlie
[Prossimo]: https://www.memorysafety.org/initiative/rustls/
-- -->

# Code of conduct

This project adopts the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
<!-- TODO(portable-rustls) UPDATE INFO FOR THIS FORK:
Please email rustls-mod@googlegroups.com to report any instance of misconduct, or if you
have any comments or questions on the Code of Conduct.
-- -->
<!-- TODO(portable-rustls) CLEANUP & UPDATE WITH MORE SPECIFIC INFO FOR THIS FORK: -->
PLEASE REPORT PRIVATELY TO A MAINTAINER OF THIS FORK IN CASE OF ANY QUESTIONS, COMMENTS, OR POSSIBLE MISCONDUCT.
