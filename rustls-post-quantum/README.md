<p align="center">
  <img width="460" height="300" src="https://raw.githubusercontent.com/rustls/rustls/main/admin/rustls-logo-web.png">
</p>

<p align="center">
Rustls is a modern TLS library written in Rust.
</p>

# rustls-post-quantum

<!-- TODO(portable-rustls) CLEANUP & IMPROVE NOTE FOR THIS FORK -->
NOTE: THIS SEPARATE "`rustls-post-quantum`" CRATE WHICH ONLY CONTAINS ALIASES IS ONLY KEPT IN THIS FORK FOR CI TESTING PURPOSES. SEE INFO BELOW FOR HOW TO USE THIS "`rustls-post-quantum`" FUNCTIONALITY WITH THE CORE `portable-rustls` CRATE IN THIS FORK.

The functionality of this crate became part of the core rustls
crate from the 0.23.22 release. When using that version of the crate,
use the `unstable-prefer-post_quantum-not-supported` Cargo feature to control whether to prefer
using post-quantum algorithms instead of using this crate.

This crate is release under the same licenses as the [main rustls crate][rustls].

[rustls]: https://crates.io/crates/rustls
[`rustls::crypto::CryptoProvider`]: https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html
