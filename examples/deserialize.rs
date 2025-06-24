use borsh::{BorshDeserialize, BorshSerialize};
use bs58;
use serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExampleData {
    pub discriminator: u8,
    pub units: u32,
}

pub fn decode_and_deserialize(
    encoded_data: &str,
) -> Result<ExampleData, Box<dyn std::error::Error>> {
    // Decode the Base58 string into bytes
    let decoded_data = bs58::decode(encoded_data).into_vec()?;

    // Deserialize the bytes into the ExampleData struct
    let deserialized_data: ExampleData = BorshDeserialize::try_from_slice(&decoded_data)?;

    Ok(deserialized_data)
}

fn main() {
    let encoded_data = "L6DKVH";

    match decode_and_deserialize(encoded_data) {
        Ok(data) => println!("\n----\nDecoded and Deserialized Data: {:?}\n----\n", data),
        Err(e) => println!("Error: {}", e),
    }
}
