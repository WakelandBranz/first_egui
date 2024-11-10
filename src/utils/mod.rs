use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

// Thank you https://stackoverflow.com/questions/59553586/how-do-i-generate-a-string-of-random-ascii-printable-characters-with-a-specific
// Didn't feel like writing it myself lol
pub fn random_string(length: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)  // From link above, this is needed in later versions
        .collect()
}