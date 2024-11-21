use std::io;
use base64::{encode, decode};

fn main() {
    loop {
        println!("Choose an operation (1-encode / 2-decode / 3-exit):");
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).unwrap();
        let operation = operation.trim();

        match operation {
            "1" => {
                println!("Enter text:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let encoded = encode(input.trim());
                println!("Base64 encoded input: {}", encoded);
            }
            "2" => {
                println!("Enter Base64 text:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                match decode(input.trim()) {
                    Ok(decoded) => {
                        let decoded_str = String::from_utf8(decoded).unwrap();
                        println!("Base64 decoded input: {}", decoded_str);
                    }
                    Err(e) => {
                        println!("Decoding error: {}", e);
                    }
                }
            }
            "3" => {
                println!("Exiting the program...");
                break;
            }
            _ => println!("Incorrect input"),
        }
    }
}
