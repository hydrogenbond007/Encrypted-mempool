use crypto::x25519::{
    derive_keypair_from_seed, generate_keypair, generate_keypair_from_rng,
    generate_keypair_hybrid,
};
use rand::{rngs::StdRng, SeedableRng};

// Derive an X25519 from seed using the extract-then-expand HKDF method from RFC 5869.
let salt = &b"some salt"[..];

let seed = [5u8; 32]; // seed is denoted as IKM in HKDF RFC 5869.
let info = &b"some app info"[..];

let (private_key1, public_key1) = derive_keypair_from_seed(Some(salt), &seed, Some(info));
let (private_key2, public_key2) = derive_keypair_from_seed(Some(salt), &seed, Some(info));
assert_eq!(public_key1, public_key2);

let (private_key, public_key) = generate_keypair();

// Generate an X25519 key pair from an RNG (in this example a SeedableRng).
let seed = [1u8; 32];
let mut rng: StdRng = SeedableRng::from_seed(seed);
let (private_key, public_key) = generate_keypair_from_rng(&mut rng);

// Generate an X25519 key pair from an RNG and a user-provided seed.
let salt = &b"some salt"[..];
// In production, ensure seed has at least 256 bits of entropy.
let seed = [5u8; 32]; // seed is denoted as IKM in HKDF RFC 5869.
let info = &b"some app info"[..];
let (private_key1, public_key1) = generate_keypair_hybrid(Some(salt), &seed, Some(info));
assert_ne!(public_key1);


