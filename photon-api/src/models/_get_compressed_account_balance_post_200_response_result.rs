/*
 * photon-indexer
 *
 * Solana indexer for general compression
 *
 * The version of the OpenAPI document: 0.45.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCompressedAccountBalancePost200ResponseResult {
    #[serde(rename = "context")]
    pub context: Box<models::Context>,
    #[serde(rename = "value")]
    pub value: i32,
}

impl GetCompressedAccountBalancePost200ResponseResult {
    pub fn new(
        context: models::Context,
        value: i32,
    ) -> GetCompressedAccountBalancePost200ResponseResult {
        GetCompressedAccountBalancePost200ResponseResult {
            context: Box::new(context),
            value,
        }
    }
}
