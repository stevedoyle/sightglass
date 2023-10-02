use sightglass_api as bench;

use wasi_crypto_guest::error::Error as WasiCryptoError;
use wasi_crypto_guest::symmetric::{
    Auth, AuthKey
};

fn sha256_hmac(data: &[u8]) -> Result<(), WasiCryptoError> {
    let key = AuthKey::generate("HMAC/SHA-256")?;
    let _tag = Auth::auth(data, &key)?;
    
    Ok(())
}

fn main() -> Result<(), WasiCryptoError> {
    let data_1mb = vec![0u8; 1*1024*1024];

    bench::start();
    sha256_hmac(&data_1mb)?;
    bench::end();

    Ok(())
}