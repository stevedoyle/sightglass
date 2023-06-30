use sightglass_api as bench;

use wasi_crypto_guest::error::Error as WasiCryptoError;
use wasi_crypto_guest::symmetric::{
    SymmetricOptions, SymmetricKey, SymmetricState,
};

fn wasi_crypto_aes_gcm_test(data: &[u8]) -> Result<(), WasiCryptoError> {
    let algorithm = "AES-256-GCM";
    let nonce = [0u8; 12];
    let mut options = SymmetricOptions::new();
    let key = SymmetricKey::generate(algorithm, Some(&options))?;

    options.set("nonce", &nonce)?;
    let mut state = SymmetricState::new(algorithm, Some(&key), Some(&options))?;
    let _ciphertext = state.encrypt(&data)?;

    Ok(())
}

fn main() -> Result<(), WasiCryptoError> {
    let data_1mb = vec![0u8; 1*1024*1024];

    bench::start();
    wasi_crypto_aes_gcm_test(&data_1mb)?;
    bench::end();

    Ok(())
}