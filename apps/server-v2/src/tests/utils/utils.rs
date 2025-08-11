const ASCII_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";

// From `random-string` crate: https://docs.rs/random-string/latest/random_string/
// Just copied and pasted just in case the crate is not available in the future.
fn generate<S: AsRef<str>>(length: usize, charset: S) -> String {
    let charset_str = charset.as_ref();

    let chars: Vec<char> = charset_str.chars().collect();
    let mut result = String::with_capacity(length);

    unsafe {
        for _ in 0..length {
            result.push(*chars.get_unchecked(fastrand::usize(0..chars.len())));
        }
    }

    result
}

pub fn random_string(length: usize) -> String {
    generate(length, ASCII_LOWERCASE)
}

pub fn random_email() -> String {
    let local_part = random_string(30);
    let domain = random_string(30);
    format!("{}@{}.com", local_part, domain)
}

pub fn random_int(min: i32, max: i32) -> i32 {
    fastrand::i32(min..max)
}
