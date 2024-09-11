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
pub struct GetValidityProofPostRequestParams {
    #[serde(rename = "hashes", skip_serializing_if = "Option::is_none")]
    pub hashes: Option<Vec<String>>,
    #[serde(rename = "newAddresses", skip_serializing_if = "Option::is_none")]
    pub new_addresses: Option<Vec<String>>,
    #[serde(
        rename = "newAddressesWithTrees",
        skip_serializing_if = "Option::is_none"
    )]
    pub new_addresses_with_trees: Option<Vec<models::AddressWithTree>>,
}

impl GetValidityProofPostRequestParams {
    pub fn new() -> GetValidityProofPostRequestParams {
        GetValidityProofPostRequestParams {
            hashes: None,
            new_addresses: None,
            new_addresses_with_trees: None,
        }
    }
}
