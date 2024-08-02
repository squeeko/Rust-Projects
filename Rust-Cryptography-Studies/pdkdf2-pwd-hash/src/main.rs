use hex_literal::hex;
use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};

use pbkdf2::{pbkdf2_hmac, pbkdf2_hmac_array, Pbkdf2};
use rand_core::OsRng;
use sha2::Sha256;

fn main() {
    let password = b"password";
    let salt = b"salt";

    // number of iterations
    let n = 600_000;

    // Expected value of generated key
    let expected = hex!("669cfe52482116fda1aa2cbe409b2f56c8e45637");

    let mut key1 = [0u8; 20];

    // dbg!(&expected);
    pbkdf2_hmac::<Sha256>(password, salt, n, &mut key1);
    assert_eq!(key1, expected);

    let key2 = pbkdf2_hmac_array::<Sha256, 20>(password, salt, n);
    assert_eq!(key2, expected);

    /*

       PBKDF2 is defined in terms of a keyed pseudo-random function (PRF). Most commonly HMAC is used as this PRF. In such cases you can use pbkdf2_hmac and pbkdf2_hmac_array functions. The former accepts a byte slice which gets filled with generated key, while the former returns an array with generated key of requested length.

    */

    // High level password hashing

    let password3 = b"hunter42";
    let salt = SaltString::generate(&mut OsRng);
    dbg!(&salt);

    // Hash password to PHC string ($pbkdf2-sha256$...)
    let password3_hash = Pbkdf2.hash_password(password3, &salt).unwrap().to_string();
    dbg!(&password3_hash);

    // Verify password against PHC string
    let parsed_hash3 = PasswordHash::new(&password3_hash).unwrap();
    dbg!(&parsed_hash3);
    assert!(Pbkdf2.verify_password(password3, &parsed_hash3).is_ok());
}

/*
[src/main.rs:37:5] &salt = SaltString("sMwPWMIN4AQ+oyK3l+1Mkw")
[src/main.rs:41:5] &password3_hash = "$pbkdf2-sha256$i=600000,l=32$sMwPWMIN4AQ+oyK3l+1Mkw$T/3cTNZYzW4pSSe3QDFcWbgtwUUMA+DINzhCzpP+Jro"
[src/main.rs:45:5] &parsed_hash3 = PasswordHash {
    algorithm: Ident(
        "pbkdf2-sha256",
    ),
    version: None,
    params: {
        Ident(
            "i",
        ): Value(
            "600000",
        ),
        Ident(
            "l",
        ): Value(
            "32",
        ),
    },
    salt: Some(
        Salt("sMwPWMIN4AQ+oyK3l+1Mkw"),
    ),
    hash: Some(
        Output("T/3cTNZYzW4pSSe3QDFcWbgtwUUMA+DINzhCzpP+Jro"),
    ),
}
*/
