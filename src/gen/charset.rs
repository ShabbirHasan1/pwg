use once_cell::sync::Lazy;

/// The default charset.
///
/// This consists of all printable ASCII characters:
///
/// * Lowercase letters: `abcdefghijklmnopqrstuvwxyz`
/// * Uppercase letters: `ABCDEFGHIJKLMNOPQRSTUVWXYZ`
/// * Numbers: `0123456789`
/// * Symbols: `!"#$%&'()*+,-./`
pub static DEFAULT_CHARSET: Lazy<Charset> = Lazy::new(|| {
    Charset::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./")
});

pub struct Charset(pub Vec<char>);

impl Charset {
    /// Create a new `Charset` from a string.
    ///
    /// This will collect all characters in the string into a `Vec<char>`.
    pub fn from(s: &str) -> Self {
        Self(s.chars().collect())
    }
}
