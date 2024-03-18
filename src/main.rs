extern crate hex;
use serde::{Serialize, Deserialize};
use serde_cbor::ser::to_vec;
use std::str::FromStr;
use serde_cbor::de::from_slice;
use hex::FromHex;

fn main() {
    let addresses = vec![
        "0x8058ad7c22fdc8788fe4cb1dac15d6e976127324",
        "0xc0D477556c25C9d67E1f57245C7453DA776B51cf",
        "0x6E37d34e35a5fF2f896eD9e76EC43e728adA1d18",
        "0x2cb21fb0a2cebb57434b1a2b89c81e5f49cd484a",
        "0xaa1decefc2b32ca6390c9773e4ecffe69a644ff7",
        "0x627a12ce1f6d42c9305e03e83fe044e8c3c1a32c",
        "0xbe14c8f33239db9699422b37f09aa86d93bb8ff6",
        "0xbaa3e3dd6eeebf87af39fc35eeccdf12537db515",
    ];

    let mut concatenated = Vec::new();

    for address in &addresses {
        // Remove the "0x" prefix and convert the address from hex to bytes
        let hex_bytes = Vec::from_hex(&address[2..]).expect("Error");
        concatenated.extend(hex_bytes);
    }
    dbg!(&concatenated);

    // Ensure the final byte array is exactly 160 bytes long
    assert_eq!(concatenated.len(), 160);

//encode
    let magic_number_ = "0xffb2637608c09e38"; // authors magic_number
    let encoded = cbor_encode(concatenated, magic_number_.to_string());

    println!("Encoded: {:?}", encoded);

// decode
     match cbor_decode(&encoded) {
         Ok(decoded_payload) => {
            //get addresses back
            let addresses: Vec<String> = decoded_payload.payload.chunks(20)
                    .map(|chunk| hex::encode(chunk))
                    .collect();

            println!("Decoded addresses:");
            for address in addresses {
                println!("0x{}", address);
            }
         }
         Err(e) => {
             eprintln!("Error: {:?}", e);
         }
     }
}

#[derive(Debug, Deserialize, Serialize)]
struct Payload {
   payload: Vec<u8>,
   magic_number: String,
}

fn cbor_encode(payload_: Vec<u8>, magic_number_: String) -> String {
    let payload = Payload {
        payload: payload_,
        magic_number: magic_number_,
    };

    let encoded = to_vec(&payload).expect("Error encoding");
    let hex_string = hex::encode(encoded);
    hex_string
}

fn cbor_decode(encoded_str: &str) -> Result<Payload, serde_cbor::Error> {
      let concatenated_hex_str = format!("{}{}", "0xff0a89c674ee7874", encoded_str);
        let extracted_substring = &concatenated_hex_str[18..];
        let encoded = hex::decode(extracted_substring).expect("Error decoding");

        dbg!(&encoded);
    from_slice(&encoded)
}

