use argon2::Argon2;

use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use rand_core::OsRng;

fn main() {
    let password = b"jermaineallgood";
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params - Argon2id v19
    let argon2 = Argon2::default();

    // Hash password to PHC string $argon2id$v19$....
    let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();

    // Verify password against PHC string.
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.

    let parsed_hash = PasswordHash::new(&password_hash);

    // dbg!(&password);
    // dbg!(&salt);
    // dbg!(&password_hash);
    // dbg!(&parsed_hash);

    assert!(argon2
        .verify_password(password, &parsed_hash.unwrap())
        .is_ok());

    // Key derivation - This API is useful for transforming a password into cryptographic keys for e.g. password-based encryption.

    let password2 = b"squeeko";
    let salt2 = b"sample salt"; // Salt should be unique per password

    let mut output_key_material = [0u8; 32]; // Can be any desired size

    Argon2::default()
        .hash_password_into(password2, salt2, &mut output_key_material)
        .unwrap();
}
