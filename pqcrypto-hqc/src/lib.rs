//! # hqc
//!
//! This crate provides bindings to and wrappers around the following
//! implementations from [PQClean][pqc]:
//!
//! * hqc-rmrs-128 - clean
//! * hqc-rmrs-192 - clean
//! * hqc-rmrs-256 - clean
//!
//! [pqc]: https://github.com/pqclean/pqclean/
//!

#![no_std]
#![allow(clippy::len_without_is_empty)]

// For no-std vectors
extern crate alloc;

// For tests
#[cfg(feature = "std")]
extern crate std;


pub mod ffi;
pub mod hqcrmrs128;
pub mod hqcrmrs192;
pub mod hqcrmrs256;

pub use crate::hqcrmrs128::{
    keypair as hqcrmrs128_keypair,
    public_key_bytes as hqcrmrs128_public_key_bytes,
    secret_key_bytes as hqcrmrs128_secret_key_bytes,
    encapsulate as hqcrmrs128_encapsulate,
    decapsulate as hqcrmrs128_decapsulate,
    ciphertext_bytes as hqcrmrs128_ciphertext_bytes,
    shared_secret_bytes as hqcrmrs128_shared_secret_bytes,
};
pub use crate::hqcrmrs192::{
    keypair as hqcrmrs192_keypair,
    public_key_bytes as hqcrmrs192_public_key_bytes,
    secret_key_bytes as hqcrmrs192_secret_key_bytes,
    encapsulate as hqcrmrs192_encapsulate,
    decapsulate as hqcrmrs192_decapsulate,
    ciphertext_bytes as hqcrmrs192_ciphertext_bytes,
    shared_secret_bytes as hqcrmrs192_shared_secret_bytes,
};
pub use crate::hqcrmrs256::{
    keypair as hqcrmrs256_keypair,
    public_key_bytes as hqcrmrs256_public_key_bytes,
    secret_key_bytes as hqcrmrs256_secret_key_bytes,
    encapsulate as hqcrmrs256_encapsulate,
    decapsulate as hqcrmrs256_decapsulate,
    ciphertext_bytes as hqcrmrs256_ciphertext_bytes,
    shared_secret_bytes as hqcrmrs256_shared_secret_bytes,
};
