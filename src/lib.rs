pub mod hash;

pub mod prelude {
    pub use crate::hash::{
        blake3::blake3_hash,
        file_render::{GetFileResult, get_data_as_bytes, get_file_uint8array_and_blob},
        hash_result::HashResult,
        hash_type::HashType,
        hasher_wrapper::{HasherWrapper, HasherWrapperConfig},
        md5::md5_hash,
        sha_256::sha_256_hash,
        sha_512::sha2_512_hash,
        sha3_256::sha3_256_hash,
        sha3_512::sha3_512_hash,
    };
}
