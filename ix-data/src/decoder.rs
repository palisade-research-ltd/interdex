use borsh::{BorshDeserialize, BorshSerialize};
use bs58;
use serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
pub struct ComputeUnitLimit {
    discriminator: u8,
    compute_unit_limit: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
pub struct ComputeUnitPrice {
    discriminator: u8,
    micro_lamports: u64,
}

pub fn decode_icd(
    encoded_data: Vec<String>,
) -> Result<(Option<u32>, Option<u64>), Box<dyn std::error::Error>> {
    let mut compute_unit_limit: Option<u32> = None;
    let mut micro_lamports: Option<u64> = None;

    for encoded_str in encoded_data {
        let decoded_data = bs58::decode(&encoded_str).into_vec()?;

        // Try to deserialize as `ComputeUnitLimit`
        if let Ok(decoded_instr_l) = ComputeUnitLimit::try_from_slice(&decoded_data[..]) {
            compute_unit_limit = Some(decoded_instr_l.compute_unit_limit);
            continue;
        }

        // Try to deserialize as `ComputeUnitPrice`
        if let Ok(decoded_instr_p) = ComputeUnitPrice::try_from_slice(&decoded_data[..]) {
            micro_lamports = Some(decoded_instr_p.micro_lamports);
            continue;
        }

        // Log an error if neither type matches
        println!(
            "\nFailed to deserialize data into ComputeUnitLimit or ComputeUnitPrice: {:?}
             \nOriginal encoded data: {:?}",
            decoded_data, &encoded_str
        );
    }

    Ok((compute_unit_limit, micro_lamports))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferInfo {
    pub source: String,
    pub destination: String,
    pub lamports: u64,
}

// -- ----------------------------------------------------------------------------- -- //

pub fn decode_instruction_data(
    encoded_data: &str,
) -> Result<TransferInfo, Box<dyn std::error::Error>> {
    // Decode Base58 string to bytes
    let decoded_data = bs58::decode(encoded_data).into_vec()?;

    let source = "";
    let destination = "";

    // Parse the next 8 bytes as lamports (u64)
    let lamports = u64::from_le_bytes(decoded_data[4..12].try_into()?);

    Ok(TransferInfo {
        source: source.to_string(),
        destination: destination.to_string(),
        lamports,
    })
}
