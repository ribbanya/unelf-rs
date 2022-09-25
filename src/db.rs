use std::io;
use std::io::{Cursor, Read};
use sha2::{Sha256, Digest};

fn hash_blob(data: impl AsRef<[u8]>) {
    // todo lazy static
    let mut sha256 = Sha256::new();
    // todo also keep the hash buffer
    sha256.update(data);
}

fn compress_blob(data: &[u8]) -> io::Result<Vec<u8>> {
    // todo compression level config setting?
    // todo reuse destination buffer in transit to db if possible
    zstd::stream::encode_all(data, 0)
}