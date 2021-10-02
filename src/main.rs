extern crate base64;

// Problem: Decode a plaintext string that is currently encoded using the following structure: ROT13 => base64 => rot13

// My Solution: Create a for loop to iterate over the string to find the Rot13, and then using a simple base64 crate for the second decryption, and back to the original algorithm for the final flag.

/**************************************************************************/
/*  Owner/Creator: @Hifumi-Sec (https://github.com/Hifumi-Sec)            */
/*  Created on: October 1st, 2021                                         */
/*  Support the dev: https://github.com/Hifumi-Sec                        */
/*  Open-Source Project: https://github.com/Hifumi-Sec/thm-rust-decrypter */
/**************************************************************************/

fn main() {
    let encoded_message = "M3I6r2IbMzq9";

    let base64_message = rot13(encoded_message.to_string()); // Decodes the second iteration, after decoding rot13 once
    let bytes = base64::decode(base64_message).unwrap(); // Converts into bytes
    let _final = String::from_utf8_lossy(&bytes); // Converts into a more readable format
    
    println!("{}", rot13(_final.to_string())); // Prints our final decoded string
}

// rot13 algorithm put together from various sources
fn rot13(encoded_message: String) -> String {
    let mut result_str = String::from("");
    // Iterates over encoded_message
    for x in encoded_message.chars() {
        let charcode = x as u32; // 32-bit - unsigned
        if x.is_lowercase() {
            // Checks if character in string is lowercase
            let check_text = 'a' as u32; // 32-bit - unsigned
            let rot_final = ((charcode - check_text + 13) % 26) + check_text;
            result_str.push(char::from_u32(rot_final).unwrap());
        } else if x.is_uppercase() {
            // Checks if character in string is uppercse
            let check_text = 'A' as u32; // 32-bit - unsigned
            let rot_final = ((charcode - check_text + 13) % 26) + check_text;
            result_str.push(char::from_u32(rot_final).unwrap());
        } else {
            result_str.push(x);
        }
    }
    result_str
}
