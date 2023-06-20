use std::time::{SystemTime, UNIX_EPOCH};

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha12Rng,
};

use super::Charset;

/// Generate a random password of a given length using a given charset.
///
/// This will use an 12-round ChaCha RNG to generate 128 bits of randomness which are then used to
/// generate the password.
///
/// # Examples
///
/// ## Generate a 16-character password
///
/// ```rust
/// use pwg::gen;
///
/// let password = gen::password(16, &gen::DEFAULT_CHARSET);
/// println!("{}", password);
/// ```
pub fn password(length: usize, charset: &Charset) -> String {
    let mut rng = ChaCha12Rng::seed_from_u64(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64,
    );

    let mut password = String::with_capacity(length);
    for _ in 0..length {
        let index = rng.next_u32() as usize % charset.0.len();
        password.push(charset.0[index]);
    }

    password
}
