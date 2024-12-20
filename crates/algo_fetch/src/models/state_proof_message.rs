/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

use serde_with::serde_as;

/// StateProofMessage : Represents the message that the state proofs are attesting to.
#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StateProofMessage {
    /// The vector commitment root on all light block headers within a state proof interval.
    #[serde_as(as = "serde_with::base64::Base64")]
    #[serde(rename = "BlockHeadersCommitment")]
    pub block_headers_commitment: Vec<u8>,
    /// The first round the message attests to.
    #[serde(rename = "FirstAttestedRound")]
    pub first_attested_round: i32,
    /// The last round the message attests to.
    #[serde(rename = "LastAttestedRound")]
    pub last_attested_round: i32,
    /// An integer value representing the natural log of the proven weight with 16 bits of precision. This value would be used to verify the next state proof.
    #[serde(rename = "LnProvenWeight")]
    pub ln_proven_weight: i32,
    /// The vector commitment root of the top N accounts to sign the next StateProof.
    #[serde_as(as = "serde_with::base64::Base64")]
    #[serde(rename = "VotersCommitment")]
    pub voters_commitment: Vec<u8>,
}

impl StateProofMessage {
    /// Represents the message that the state proofs are attesting to.
    pub fn new(block_headers_commitment: Vec<u8>, first_attested_round: i32, last_attested_round: i32, ln_proven_weight: i32, voters_commitment: Vec<u8>) -> StateProofMessage {
        StateProofMessage {
            block_headers_commitment,
            first_attested_round,
            last_attested_round,
            ln_proven_weight,
            voters_commitment,
        }
    }
}
