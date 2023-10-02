use sightglass_api as bench;

use wasi_crypto_guest::error::Error as WasiCryptoError;
use wasi_crypto_guest::signatures::SignatureKeyPair;
use wasi_crypto_guest::prelude::Signature;

fn wasi_crypto_verify(key_pair: &SignatureKeyPair, msg: &[u8], sig: &Signature) 
    -> Result<(), WasiCryptoError> {
    key_pair.publickey()?.signature_verify(msg, sig)?;

    Ok(())
}

fn main() -> Result<(), WasiCryptoError> {
    const MESSAGE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz1234567890";
    let rsa_kp = SignatureKeyPair::generate("RSA_PKCS1_2048_SHA256")?;
    let sig = rsa_kp.sign(MESSAGE)?;

    bench::start();
    wasi_crypto_verify(&rsa_kp, &MESSAGE, &sig)?;
    bench::end();

    Ok(())
}