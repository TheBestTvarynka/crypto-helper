use picky::hash::HashAlgorithm;
use time::OffsetDateTime;

use crate::crypto_helper::RSA_HASH_ALGOS;

pub fn format_date_time(datetime: &OffsetDateTime) -> String {
    format!(
        "{}:{}:{} {}.{}.{}",
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
        datetime.day(),
        datetime.month(),
        datetime.year()
    )
}

pub fn compate_hash_algorithms(hash_1: &str, hash_2: &HashAlgorithm) -> bool {
    match hash_2 {
        HashAlgorithm::MD5 => RSA_HASH_ALGOS[0] == hash_1,
        HashAlgorithm::SHA1 => RSA_HASH_ALGOS[1] == hash_1,
        HashAlgorithm::SHA2_224 => RSA_HASH_ALGOS[2] == hash_1,
        HashAlgorithm::SHA2_256 => RSA_HASH_ALGOS[3] == hash_1,
        HashAlgorithm::SHA2_384 => RSA_HASH_ALGOS[4] == hash_1,
        HashAlgorithm::SHA2_512 => RSA_HASH_ALGOS[5] == hash_1,
        HashAlgorithm::SHA3_384 => RSA_HASH_ALGOS[6] == hash_1,
        HashAlgorithm::SHA3_512 => RSA_HASH_ALGOS[7] == hash_1,
        _ => false,
    }
}

pub fn hash_algorithm_from_str(raw: &str) -> Option<HashAlgorithm> {
    if RSA_HASH_ALGOS[0] == raw {
        Some(HashAlgorithm::MD5)
    } else if RSA_HASH_ALGOS[1] == raw {
        Some(HashAlgorithm::SHA1)
    } else if RSA_HASH_ALGOS[2] == raw {
        Some(HashAlgorithm::SHA2_224)
    } else if RSA_HASH_ALGOS[3] == raw {
        Some(HashAlgorithm::SHA2_256)
    } else if RSA_HASH_ALGOS[4] == raw {
        Some(HashAlgorithm::SHA2_384)
    } else if RSA_HASH_ALGOS[5] == raw {
        Some(HashAlgorithm::SHA2_512)
    } else if RSA_HASH_ALGOS[6] == raw {
        Some(HashAlgorithm::SHA3_384)
    } else if RSA_HASH_ALGOS[7] == raw {
        Some(HashAlgorithm::SHA3_512)
    } else {
        None
    }
}
