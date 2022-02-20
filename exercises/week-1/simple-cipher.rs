extern crate rand;
use rand::{thread_rng, Rng};
use std::ops::Range;
static RANGE: Range<u8> = b'a'..(b'z' + 1);
pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = thread_rng();
    let key: String = (0..100).map(|_| (rng.gen_range(RANGE.start, RANGE.end)) as char).collect();
    let encoded_string = encode(&key, s);
    (key, encoded_string.unwrap())
}
pub fn encode(key: &str, s: &str) -> Option<String> {
    transform(key, s, encode_char)
}
pub fn decode(key: &str, s: &str) -> Option<String> {
    transform(key, s, decode_char)
}
fn transform(key: &str, s: &str, transform_char: fn(char, char) -> char) -> Option<String> {
    if { is_valid_string(key) && is_valid_string(s) } {
        Some(s.chars().zip(key.chars().cycle()).map(|(c, k)| transform_char(c, k)).collect())
    } else {
        None
    }
}
fn encode_char(c: char, key: char) -> char {
    ((c as u8 + key as u8 - 2 * RANGE.start) % (RANGE.len() as u8) + RANGE.start) as char
}
fn decode_char(c: char, key: char) -> char {
    (((RANGE.len() as u8) + c as u8 - key as u8) % (RANGE.len() as u8) + RANGE.start) as char
}
fn is_valid_string(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| RANGE.start <= (c as u8) && (c as u8) < RANGE.end)
}