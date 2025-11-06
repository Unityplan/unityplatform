use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Username validation regex: alphanumeric and underscores only
    pub static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
}
