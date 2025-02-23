#![cfg(any(feature = "ring", cfg_aws_lc_rs_not_supported))]

//! Note that the default test runner builds each test file into a separate
//! executable, and runs tests in an indeterminate order.  That restricts us
//! to doing all the desired tests, in series, in one function.

use portable_rustls as rustls; // TEST IMPORT WORKAROUND for this fork
#[cfg(all(cfg_aws_lc_rs_not_supported, not(feature = "ring")))]
use rustls::crypto::aws_lc_rs as provider;
#[cfg(all(feature = "ring", not(cfg_aws_lc_rs_not_supported)))]
use rustls::crypto::ring as provider;
#[cfg(all(feature = "ring", cfg_aws_lc_rs_not_supported))]
use rustls::crypto::ring as provider;
use rustls::crypto::CryptoProvider;
use rustls::ClientConfig;

mod common;
use crate::common::*;

#[test]
fn test_process_provider() {
    if dbg!(cfg!(all(feature = "ring", cfg_aws_lc_rs_not_supported))) {
        test_explicit_choice_required();
    } else if dbg!(cfg!(all(feature = "ring", not(cfg_aws_lc_rs_not_supported)))) {
        test_ring_used_as_implicit_provider();
    } else if dbg!(cfg!(all(cfg_aws_lc_rs_not_supported, not(feature = "ring")))) {
        test_aws_lc_rs_used_as_implicit_provider();
    } else {
        panic!("fix feature combinations");
    }
}

fn test_explicit_choice_required() {
    assert!(CryptoProvider::get_default().is_none());
    provider::default_provider()
        .install_default()
        .expect("cannot install");
    CryptoProvider::get_default().expect("provider missing");
    provider::default_provider()
        .install_default()
        .expect_err("install succeeded a second time");
    CryptoProvider::get_default().expect("provider missing");

    // does not panic
    finish_client_config(KeyType::Rsa2048, ClientConfig::builder());
}

fn test_ring_used_as_implicit_provider() {
    assert!(CryptoProvider::get_default().is_none());

    // implicitly installs ring provider
    finish_client_config(KeyType::Rsa2048, ClientConfig::builder());

    let default = CryptoProvider::get_default().expect("provider missing");
    let debug = format!("{default:?}");
    assert!(debug.contains("secure_random: Ring"));

    let builder = ClientConfig::builder();
    assert_eq!(format!("{:?}", builder.crypto_provider()), debug);
}

fn test_aws_lc_rs_used_as_implicit_provider() {
    assert!(CryptoProvider::get_default().is_none());

    // implicitly installs aws-lc-rs provider
    finish_client_config(KeyType::Rsa2048, ClientConfig::builder());

    let default = CryptoProvider::get_default().expect("provider missing");
    let debug = format!("{default:?}");
    assert!(debug.contains("secure_random: AwsLcRs"));

    let builder = ClientConfig::builder();
    assert_eq!(format!("{:?}", builder.crypto_provider()), debug);
}
