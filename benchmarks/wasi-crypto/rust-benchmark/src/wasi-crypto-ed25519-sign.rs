use sightglass_api as bench;

use wasi_crypto_guest::error::Error as WasiCryptoError;
use wasi_crypto_guest::signatures::SignatureKeyPair;

fn wasi_crypto_sign(key_pair: &SignatureKeyPair, msg: &[u8]) 
    -> Result<(), WasiCryptoError> {
    let _signature = key_pair.sign(msg)?;
    
    Ok(())
}

fn main() -> Result<(), WasiCryptoError> {
    const MESSAGE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz1234567890";
    let rsa_kp = SignatureKeyPair::generate("Ed25519")?;

    bench::start();
    wasi_crypto_sign(&rsa_kp, &MESSAGE)?;
    bench::end();

    Ok(())
}