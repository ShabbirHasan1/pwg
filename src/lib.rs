//! # A password generator
//!
//! This module contains a password generator. It is a collection of highly customizable functions
//! that may be used to generate passwords.
//!
//! ## Examples
//!
//! ### Generate a password
//!
//! ```rust
//! use pwg::gen::{generate_password, DEFAULT_CHARSET};
//!
//! let password = generate_password(16, &DEFAULT_CHARSET);
//! println!("{}", password);
//! ```
//!
//! ### Generate a password with a custom charset
//!
//! ```rust
//! use pwg::gen::{generate_password, Charset};
//!
//! let charset = Charset::from("0123456789abcdef");
//! let password = generate_password(16, &charset);
//! println!("{}", password);
//! ```
//!
//! ## Details
//!
//! ### Randomness
//!
//! The password generator uses the [`rand_chacha`](https://crates.io/crates/rand_chacha) crate to
//! generate cryptographically secure numbers.
//!
//! The generator utilizes 12 rounds of the ChaCha algorithm to generate 256 bits of randomness
//! which are then used to generate the password.
//!
//! ### Charset
//!
//! A charset is a collection of characters that the password generator uses to generate the
//! password. This means that the password generator will only use characters from the charset to
//! generate the password (multiple instances of a character are allowed).
//!
//! Internally, it is a wrapper around a `Vec<char>`. The default charset is defined in
//! `DEFAULT_CHARSET` and contains all printable ASCII characters:
//!
//! * Lowercase letters: `abcdefghijklmnopqrstuvwxyz`
//! * Uppercase letters: `ABCDEFGHIJKLMNOPQRSTUVWXYZ`
//! * Numbers: `0123456789`
//! * Symbols: `!"#$%&'()*+,-./`
//!
//! ### Password length
//!
//! The password length is the number of characters in the password. The password generator will
//! generate a password with exactly this number of characters.
//!
//! ### Password strength
//!
//! The generator ensures that the generated password has a strength of at least 128 bits. This
//! means that the password is resistant to brute-force attacks that use less than 2^128 guesses,
//! which is infeasible with current technology.
//!
//! However, 128 bits of strength will do nothing if the password length is too short, so you should
//! always use a password length of at least 8 characters.
//!
//! ## Performance
//!
//! The main bottleneck of the password generator is the generation of the random numbers.
//!
//! If you have compiled the crate with `rand_chacha`'s `simd` feature, the generator will use the
//! SIMD implementation of ChaCha, which is significantly faster than the scalar implementation.
//!
//! We try to balance security and performance, so we have enabled the `simd` feature by default.

// 👆 Slightly modified docs from gen/mod.rs

pub use gen::{generate_password, Charset, DEFAULT_CHARSET};

pub mod gen;
pub mod http;
