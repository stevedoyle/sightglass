use sightglass_api as bench;

use wasi_crypto_guest::error::Error as WasiCryptoError;
use wasi_crypto_guest::symmetric::{
    Hash
};


fn sha256_hash(data: &[u8]) -> Result<(), WasiCryptoError> {
    let _tag = Hash::hash("SHA-256", data, 32, None)?;
    
    Ok(())
}

fn main() -> Result<(), WasiCryptoError> {
    let data_1mb = vec![0u8; 1*1024*1024];

    bench::start();
    sha256_hash(&data_1mb)?;
    bench::end();

    Ok(())
}