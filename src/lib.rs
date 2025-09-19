pub mod hash;

pub mod prelude {
    pub use crate::hash::{
        blake3::blake3_hash,
        hash_result::HashResult,
        hash_type::HashType,
        hash_worker::HashWorker,
        hasher_wrapper::{HasherWrapper, HasherWrapperConfig},
        md5::md5_hash,
        sha2_256::sha2_256_hash,
        sha2_512::sha2_512_hash,
        sha3_256::sha3_256_hash,
        sha3_512::sha3_512_hash,
    };
}
