use std::collections::HashMap;

use serde::Deserialize;

// --- Quote API response (GET /v3/quote) ---

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct QuoteApiResponse {
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "type")]
    pub quote_type: String,
    pub status: String,
    #[serde(rename = "quoteId")]
    pub quote_id: String,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    #[serde(rename = "approvalType")]
    pub approval_type: ApprovalType,
    #[serde(rename = "nativeToken")]
    pub native_token: String,
    pub taker: String,
    pub receiver: String,
    pub expiry: u64,
    pub slippage: f64,
    #[serde(rename = "gasFee")]
    pub gas_fee: GasFeeResponse,
    #[serde(rename = "buyTokens")]
    pub buy_tokens: HashMap<String, ResponseBuyToken>,
    #[serde(rename = "sellTokens")]
    pub sell_tokens: HashMap<String, ResponseSellToken>,
    #[serde(rename = "settlementAddress")]
    pub settlement_address: String,
    #[serde(rename = "approvalTarget")]
    pub approval_target: String,
    #[serde(rename = "requiredSignatures")]
    pub required_signatures: Vec<String>,
    #[serde(rename = "priceImpact")]
    pub price_impact: Option<f64>,
    #[serde(rename = "partnerFee")]
    pub partner_fee: Option<HashMap<String, String>>,
    #[serde(rename = "protocolFee")]
    pub protocol_fee: Option<HashMap<String, String>>,
    #[serde(default)]
    pub warnings: Vec<PriceWarning>,
    pub info: Option<String>,
    pub tx: Option<TxData>,
    pub solana_tx: Option<String>,
    pub blockhash: Option<String>,
    pub makers: Vec<String>,
    #[serde(rename = "toSign")]
    pub to_sign: Option<serde_json::Value>,
    #[serde(rename = "onchainOrderType")]
    pub onchain_order_type: Option<String>,
    #[serde(rename = "partialFillOffset")]
    pub partial_fill_offset: Option<i64>,
    #[serde(rename = "guaranteeBoost")]
    pub guarantee_boost: Option<String>,
}

// --- Enums ---

#[derive(Deserialize, Debug)]
pub enum ApprovalType {
    Standard,
    Permit,
    Permit2,
}

// --- Nested response types ---

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct GasFeeResponse {
    pub native: String,
    pub usd: Option<f64>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ResponseBuyToken {
    pub amount: String,
    pub decimals: u8,
    #[serde(rename = "priceUsd")]
    pub price_usd: Option<f64>,
    pub symbol: String,
    #[serde(rename = "minimumAmount")]
    pub minimum_amount: String,
    pub price: Option<f64>,
    #[serde(rename = "priceBeforeFee")]
    pub price_before_fee: Option<f64>,
    #[serde(rename = "amountBeforeFee")]
    pub amount_before_fee: Option<String>,
    #[serde(rename = "deltaFromExpected")]
    pub delta_from_expected: Option<f64>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ResponseSellToken {
    pub amount: String,
    pub decimals: u8,
    #[serde(rename = "priceUsd")]
    pub price_usd: Option<f64>,
    pub symbol: String,
    pub price: Option<f64>,
    #[serde(rename = "priceBeforeFee")]
    pub price_before_fee: Option<f64>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct PriceWarning {
    pub code: i32,
    pub message: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct TxData {
    #[serde(rename = "chainId")]
    pub chain_id: Option<u64>,
    pub from: Option<String>,
    pub to: String,
    pub value: String,
    pub data: String,
    pub gas: Option<u64>,
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<u64>,
}

// --- Error response ---

#[derive(Deserialize)]
pub struct ErrorApiResponse {
    pub error: ApiError,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ApiError {
    #[serde(rename = "errorCode")]
    pub error_code: i32,
    pub message: Option<String>,
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
}
