use drand_core::HttpClient;

#[tokio::main]
async fn main() {
    let msg = b"using drand for basic data tlock encryption".to_vec();
    let client: HttpClient =
        "https://api.drand.sh/dbd506d6ef76e5f386f41c651dcb808c5bcbd75471cc4eafa3f4df7ad4e4c493"
            .try_into()
            .unwrap();
    let info = client.chain_info().await.unwrap();
    let round = 100;

    // Encryption as binary.
    let mut encrypted = vec![];
    tlock_age::encrypt(
        &mut encrypted,
        msg.as_slice(),
        &info.hash(),
        &info.public_key(),
        round,
    )
    .unwrap();

    // Decrypting the message. It requires the round signature, here retrieved from the beacon above.
    let mut decrypted = vec![];
    let signature = client.get(round).await.unwrap().signature();

    tlock_age::decrypt(
        &mut decrypted,
        encrypted.as_slice(),
        &info.hash(),
        &signature,
    )
    .unwrap();
    let decrypted = std::str::from_utf8(&decrypted).unwrap();

    println!("{decrypted}");
}

