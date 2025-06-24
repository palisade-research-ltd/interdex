//! Data Structures

#![allow(warnings)]

use crate::types::TransactionType;
use serde::{Deserialize, Serialize};
use serde_json::json;

// ------------------------------------------------------- --------------------------------- -- //
// ------------------------------------------------------- --------------------------------- -- //

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaResponse2 {
    pub id: i64,
    pub jsonrpc: String,
    pub result: Option<BlockResult>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockResult {
    pub block_height: Option<i64>,
    pub block_time: Option<i64>,
    pub blockhash: Option<String>,
    pub parent_slot: Option<u64>,
    pub previous_blockhash: Option<String>,
    pub transactions: Option<Vec<Transactions>>
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    pub meta: Option<TransactionMeta2>,
    pub transaction: Option<Transaction>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMeta2 {
    pub err: Option<serde_json::Value>,
    pub fee: Option<u64>,
    pub pre_balances: Option<Vec<u64>>,
    pub post_balances: Option<Vec<u64>>,
    pub inner_instructions: Option<Vec<InnerInstruction>>,
    pub log_messages: Option<Vec<String>>,
    pub pre_token_balances: Option<Vec<TokenBalance>>,
    pub post_token_balances: Option<Vec<TokenBalance>>,
    pub rewards: Option<Vec<Reward>>,
    pub status: Option<TransactionStatus>,
    pub compute_units_consumed: Option<u64>,
    pub loaded_addresses: Option<LoadedAddresses>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub message: Option<Message>,
    pub signatures: Vec<String>,
}

// ------------------------------------------------------- Solana Recent Prioritization Fees -- //
// ------------------------------------------------------- --------------------------------- -- //

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaResponse {
    pub jsonrpc: String,
    pub result: Option<Vec<SolanaResult>>,
    pub id: i64,
}


#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaResult {
    pub slot: Option<i64>,
    pub prioritization_fee: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct priorityFeeRecentResponse {
    pub slots: Option<Vec<i64>>,
    pub fees: Option<Vec<u64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct priorityFeeRecentComodin {
    pub content: String,
}

// -------------------------------------------------------------------- Enriched Transaction -- //
// -------------------------------------------------------------------- -------------------- -- //

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnhancedTransactionResponse {
    pub description: String,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub source: Source,
    pub fee: i64,
    pub fee_payer: String,
    pub signature: String,
    pub slot: u64,
    pub timestamp: i64,
    pub native_transfers: Option<Vec<NativeTransfer>>,
    pub token_transfers: Option<Vec<TokenTransfer>>,
    pub account_data: Option<Vec<AccountData>>,
    pub instructions: Option<Vec<Instruction>>,
    #[serde(rename = "transactionError")]
    pub transaction_error: Option<TransactionError>,
    pub events: Option<Events>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Source {
    form_function,
    // Add other sources as needed
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeTransfer {
    pub from_user_account: String,
    pub to_user_account: String,
    pub amount: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenTransfer {
    pub from_user_account: String,
    pub to_user_account: String,
    pub from_token_account: String,
    pub to_token_account: String,
    pub token_amount: i64,
    pub mint: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountData {
    pub account: String,
    pub native_balance_change: i64,
    pub token_balance_changes: Vec<TokenBalanceChange>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalanceChange {
    pub user_account: String,
    pub token_account: String,
    pub mint: String,
    pub raw_token_amount: RawTokenAmount,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawTokenAmount {
    //#[serde(rename = "tokenAmount")]
    pub token_amount: String,
    pub decimals: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransactionError {
    pub error: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Events {
    pub nft: Option<NftEvent>,
    pub swap: Option<SwapEvent>,
    pub compressed: Option<CompressedEvent>,
    //#[serde(rename = "distributeCompressionRewards")]
    pub distribute_compression_rewards: Option<DistributeCompressionRewards>,
    //#[serde(rename = "setAuthority")]
    pub set_authority: Option<SetAuthorityEvent>,
}

// NFT Event structures
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NftEventType {
    nft_bid,
    // Add other NFT event types
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NftEvent {
    pub description: String,
    #[serde(rename = "type")]
    pub event_type: NftEventType,
    pub source: Source,
    pub amount: u64,
    pub fee: u64,
    pub fee_payer: String,
    pub signature: String,
    pub slot: u64,
    pub timestamp: i64,
    pub sale_type: String,
    pub buyer: String,
    pub seller: String,
    pub staker: String,
    pub nfts: Vec<Nft>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nft {
    pub mint: String,
    //#[serde(rename = "tokenStandard")]
    pub token_standard: TokenStandard,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenStandard {
    non_fungible,
    // Add other token standards
}

// Swap Event structures
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapEvent {
    pub native_input: NativeIO,
    pub native_output: NativeIO,
    pub token_inputs: Vec<TokenIO>,
    pub token_outputs: Vec<TokenIO>,
    pub token_fees: Vec<TokenIO>,
    pub native_fees: Vec<NativeIO>,
    pub inner_swaps: Vec<InnerSwap>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeIO {
    pub account: String,
    pub amount: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenIO {
    pub user_account: String,
    pub token_account: String,
    pub mint: String,
    pub raw_token_amount: RawTokenAmount,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerSwap {
    pub program_info: ProgramInfo,
    pub token_inputs: Vec<TokenTransfer>,
    pub token_outputs: Vec<TokenTransfer>,
    pub token_fees: Vec<TokenTransfer>,
    pub native_fees: Vec<NativeTransfer>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramInfo {
    pub source: String,
    pub account: String,
    pub program_name: String,
    pub instruction_name: String,
}

// Compression Event structures
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompressedEvent {
    #[serde(rename = "type")]
    pub event_type: CompressedEventType,
    pub tree_id: String,
    pub asset_id: String,
    pub leaf_index: i32,
    pub instruction_index: i32,
    pub inner_instruction_index: i32,
    pub new_leaf_owner: String,
    pub old_leaf_owner: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CompressedEventType {
    compressed_nft_mint,
    // Add other compressed event types
}

#[derive(Debug, Clone, Deserialize)]
pub struct DistributeCompressionRewards {
    pub amount: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAuthorityEvent {
    pub account: String,
    pub from: String,
    pub to: String,
    pub instruction_index: i32,
    pub inner_instruction_index: i32,
}

// ----------------------------------------------------------------------------- Transaction -- //
// ----------------------------------------------------------------------------- ----------- -- //

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponse {
    pub jsonrpc: String,
    pub result: Option<TransactionResult>,
    pub id: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResult {
    pub blockTime: Option<i64>,
    pub meta: TransactionMeta,
    pub slot: Option<u64>,
    pub transaction: Transaction,
    pub version: Option<u8>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransactionStatus {
    pub ok: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMeta {
    pub err: Option<serde_json::Value>,
    pub fee: Option<u64>,
    pub pre_balances: Option<Vec<u64>>,
    pub post_balances: Option<Vec<u64>>,
    pub inner_instructions: Option<Vec<InnerInstruction>>,
    pub log_messages: Option<Vec<String>>,
    pub pre_token_balances: Option<Vec<TokenBalance>>,
    pub post_token_balances: Option<Vec<TokenBalance>>,
    pub rewards: Option<Vec<Reward>>,
    pub status: Option<TransactionStatus>,
    pub compute_units_consumed: Option<u64>,
    pub loaded_addresses: Option<LoadedAddresses>,
}

// --------------------------------------------------------------------------------- Message -- //
// --------------------------------------------------------------------------------- ------- -- //

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub account_keys: Option<Vec<String>>,
    pub header: MessageHeader,
    pub instructions: Option<Vec<Instruction>>,
    pub recent_blockhash: Option<String>,
    pub address_table_lookups: Option<Vec<AddressTableLookup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageHeader {
    pub num_required_signatures: Option<u8>,
    pub num_readonly_signed_accounts: Option<u8>,
    pub num_readonly_unsigned_accounts: Option<u8>,
}

// ----------------------------------------------------------------------------- Instruction -- //
// ----------------------------------------------------------------------------- ----------- -- //

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    pub program_id_index: Option<u64>,
    pub accounts: Option<Vec<u64>>,
    pub data: Option<String>,
    pub stack_height: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerInstruction {
    pub index: u8,
    pub instructions: Vec<Instruction>,
}

// ------------------------------------------------------------------------------- Addresses -- //
// ------------------------------------------------------------------------------- --------- -- //

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressTableLookup {
    pub account_key: Option<String>,
    pub writable_indexes: Option<Vec<u8>>,
    pub readonly_indexes: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadedAddresses {
    pub writable: Vec<String>,
    pub readonly: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalance {
    pub account_index: u8,
    pub mint: String,
    pub owner: String,
    pub program_id: String,
    pub ui_token_amount: UiTokenAmount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTokenAmount {
    pub amount: String,
    pub decimals: u8,
    pub ui_amount: Option<f64>,
    pub ui_amount_string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    // Add fields if rewards are used
}

// --------------------------------------------------------------------------- Priority Fees -- //
// --------------------------------------------------------------------------- ------------- -- //

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct priorityFeeEstimateResponse {
    pub jsonrpc: String,
    pub result: Option<priorityFeeEstimateResult>,
    pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct priorityFeeEstimateResult {
    pub priority_fee_estimate: Option<f64>,
    pub priority_fee_levels: Option<priorityFeeLevels>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct priorityFeeLevels {
    pub min: Option<f64>,
    pub low: Option<f64>,
    pub medium: Option<f64>,
    pub high: Option<f64>,
    pub very_high: Option<f64>,
    pub unsafe_max: Option<f64>,
}
