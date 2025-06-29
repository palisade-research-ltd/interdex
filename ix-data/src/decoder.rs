use borsh::{BorshDeserialize, BorshSerialize};
use bs58;
//use hex::encode as hex_encode;
//use hex::FromHexError;
use serde::{Deserialize, Serialize};
//use serde_json::json;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComputeUnitLimit {
    discriminator: u8,
    compute_unit_limit: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComputeUnitPrice {
    discriminator: u8,
    micro_lamports: u64,
}

pub fn decode_icd(
    encoded_data: Vec<String>,
) -> Result<(Option<u32>, Option<u64>), Box<dyn std::error::Error>> {
    let mut compute_unit_limit: u32 = 0;
    let mut micro_lamports: u64 = 0;

    for encoded_str in encoded_data {
        let decoded_data = bs58::decode(&encoded_str).into_vec()?.clone();

        // Try to deserialize as `ComputeUnitLimit`
        if let Ok(decoded_instr_l) = ComputeUnitLimit::try_from_slice(&decoded_data[..]) {
            compute_unit_limit = decoded_instr_l.compute_unit_limit;
            // println!("encoded_str: {:?}", &encoded_str);
            continue;
        }

        // Try to deserialize as `ComputeUnitPrice`
        if let Ok(decoded_instr_p) = ComputeUnitPrice::try_from_slice(&decoded_data[..]) {
            micro_lamports = decoded_instr_p.micro_lamports;
            // println!("Decoded as micro_lamports: {:?}", micro_lamports);
            continue;
        }

        // Log an error if neither type matches
        println!(
            "\n Failed to deserialize data into ComputeUnitLimit or ComputeUnitPrice: {:?}
             \n Original encoded data {:?}",
            decoded_data, &encoded_str
        );
    }

    Ok((Some(compute_unit_limit), Some(micro_lamports)))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferInfo {
    pub source: String,
    pub destination: String,
    pub lamports: u64,
}

// -- ----------------------------------------------------------------------------- -- //
// -- ----------------------------------------------------------------------------- -- //

pub fn decode_instruction_data(
    encoded_data: &str,
) -> Result<TransferInfo, Box<dyn std::error::Error>> {
    // Decode Base58 string to bytes
    let decoded_data = bs58::decode(encoded_data).into_vec()?;

    let source = "";
    let destination = "";

    // Parse the first 4 bytes as the discriminator (u32)
    // let discriminator = u32::from_le_bytes(decoded_data[0..4].try_into()?);

    // Parse the next 8 bytes as lamports (u64)
    let lamports = u64::from_le_bytes(decoded_data[4..12].try_into()?);

    Ok(TransferInfo {
        source: source.to_string(),
        destination: destination.to_string(),
        lamports,
    })
}
