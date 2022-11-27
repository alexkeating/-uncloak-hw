#[macro_use]
extern crate anyhow;
use anyhow::Result;

// Can only handle A-za-z
pub fn encrypt<'a>(msg: &'a str, key: &'a str) -> Result<String> {
    // 1. iterate through the msg and shift
    // 2. position + 1 % length should give the value of the key
    let key_length = key.len();
    let char_indices = msg.char_indices();
    let mut encrypted_str = "".to_string();
    for (pos, ch) in char_indices {
        let shifted_key_str;
        println!("{}", pos);
        if pos < key_length {
            shifted_key_str = match key.get(pos..pos + 1) {
                None => "",
                Some(char) => char,
            };
        } else {
            let key_index = (pos).rem_euclid(key_length);
            shifted_key_str = match key.get(key_index..key_index + 1) {
                None => "",
                Some(char) => char,
            };
        };
        println!("{}", shifted_key_str);
        let shifted_key_char = match shifted_key_str.chars().next() {
            None => '❤',
            Some(char) => char,
        };
        if shifted_key_char == '❤' {
            return Err(anyhow!("Failed to encrypt: improper key shifting"));
        };
        let mut shifted_char =
            ch.to_ascii_uppercase() as u8 + (shifted_key_char.to_ascii_uppercase() as u8 - 65);
        // if greater than max
        if shifted_char > 90 {
            shifted_char = (shifted_char - 91) + 65
        }
        encrypted_str.push(shifted_char as char);
    }
    Ok(encrypted_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Msg bigger than key
    #[test]
    fn base() {
        let msg = "attackatdawn";
        let key = "LEMON";
        let result = encrypt(msg, key).unwrap();
        assert_eq!(result, "LXFOPVEFRNHR");
    }

    // Key bigger than msg
    #[test]
    fn base_two() {
        let msg = "Hellosan";
        let key = "Myerhoffer";
        let result = encrypt(msg, key).unwrap();
        assert_eq!(result, "TCPCVGFS");
    }
}
