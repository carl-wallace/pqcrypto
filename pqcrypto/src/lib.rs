/// Post-Quantum cryptographic primitives
///
/// Packages the [PQClean][pqclean] project as Rust crates
///
/// [pqclean]: https://github.com/PQClean/PQClean/
pub use pqcrypto_traits as traits;

pub mod prelude {
    pub use pqcrypto_traits::kem::{
        Ciphertext as _, PublicKey as _, SecretKey as _, SharedSecret as _,
    };
    pub use pqcrypto_traits::sign::{
        DetachedSignature as _, PublicKey as _, SecretKey as _, SignedMessage as _,
    };
}

pub mod kem {
    #[cfg(feature = "pqcrypto-kyber")]
    pub use pqcrypto_kyber::{
        kyber1024, kyber102490s, kyber512, kyber51290s, kyber768, kyber76890s,
    };
}

pub mod sign {
    #[cfg(feature = "pqcrypto-dilithium")]
    pub use pqcrypto_dilithium::{dilithium2, dilithium3, dilithium5};
    #[cfg(feature = "pqcrypto-falcon")]
    pub use pqcrypto_falcon::{falcon1024, falcon512};
    #[cfg(feature = "pqcrypto-sphincsplus")]
    pub use pqcrypto_sphincsplus::{
        sphincsharaka128frobust, sphincsharaka128fsimple, sphincsharaka128srobust,
        sphincsharaka128ssimple, sphincsharaka192frobust, sphincsharaka192fsimple,
        sphincsharaka192srobust, sphincsharaka192ssimple, sphincsharaka256frobust,
        sphincsharaka256fsimple, sphincsharaka256srobust, sphincsharaka256ssimple,
        sphincssha256128frobust, sphincssha256128fsimple, sphincssha256128srobust,
        sphincssha256128ssimple, sphincssha256192frobust, sphincssha256192fsimple,
        sphincssha256192srobust, sphincssha256192ssimple, sphincssha256256frobust,
        sphincssha256256fsimple, sphincssha256256srobust, sphincssha256256ssimple,
        sphincsshake256128frobust, sphincsshake256128fsimple, sphincsshake256128srobust,
        sphincsshake256128ssimple, sphincsshake256192frobust, sphincsshake256192fsimple,
        sphincsshake256192srobust, sphincsshake256192ssimple, sphincsshake256256frobust,
        sphincsshake256256fsimple, sphincsshake256256srobust, sphincsshake256256ssimple,
    };
}
