use serde::Deserialize;
use std::fs;
use std::error::Error;
use ethnum::U256; // Changed from primitive_types::U256 to ethnum::U256
use expander_compiler::frontend::*;
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Jwt {
    pub jwt: Vec<String>,
    pub jwt_header_with_separator: Vec<String>,
    pub jwt_payload: Vec<String>,
    pub public_inputs_hash: String,
    pub header_len_with_separator: String,
    pub signature: Vec<String>,
    pub pubkey_modulus: Vec<String>,
    pub aud_field: Vec<String>,
    pub aud_field_string_bodies: Vec<String>,
    pub aud_field_len: String,
    pub aud_index: String,
    pub aud_value_index: String,
    pub aud_colon_index: String,
    pub aud_name: Vec<String>,
    pub uid_field: Vec<String>,
    pub uid_field_string_bodies: Vec<String>,
    pub uid_field_len: String,
    pub uid_index: String,
    pub uid_name_len: String,
    pub uid_value_index: String,
    pub uid_value_len: String,
    pub uid_colon_index: String,
    pub uid_name: Vec<String>,
    pub uid_value: Vec<String>,
    pub ev_field: Vec<String>,
    pub ev_field_len: String,
    pub ev_index: String,
    pub ev_value_index: String,
    pub ev_value_len: String,
    pub ev_colon_index: String,
    pub ev_name: Vec<String>,
    pub ev_value: Vec<String>,
    pub iss_field: Vec<String>,
    pub iss_field_string_bodies: Vec<String>,
    pub iss_field_len: String,
    pub iss_index: String,
    pub iss_value_index: String,
    pub iss_value_len: String,
    pub iss_colon_index: String,
    pub iss_name: Vec<String>,
    pub iss_value: Vec<String>,
    pub nonce_field: Vec<String>,
    pub nonce_field_string_bodies: Vec<String>,
    pub nonce_field_len: String,
    pub nonce_index: String,
    pub nonce_value_index: String,
    pub nonce_value_len: String,
    pub nonce_colon_index: String,
    pub nonce_name: Vec<String>,
    pub nonce_value: Vec<String>,
    pub temp_pubkey: Vec<String>,
    pub jwt_randomness: String,
    pub pepper: String,
    pub jwt_num_sha2_blocks: String,
    pub iat_field: Vec<String>,
    pub iat_field_len: String,
    pub iat_index: String,
    pub iat_value_index: String,
    pub iat_value_len: String,
    pub iat_colon_index: String,
    pub iat_name: Vec<String>,
    pub iat_value: Vec<String>,
    pub exp_date: String,
    pub exp_delta: String,
    pub b64_payload_len: String,
    pub jwt_len_bit_encoded: Vec<String>,
    pub padding_without_len: Vec<String>,
    pub temp_pubkey_len: String,
    pub private_aud_value: Vec<String>,
    pub override_aud_value: Vec<String>,
    pub private_aud_value_len: String,
    pub override_aud_value_len: String,
    pub use_aud_override: String,
    pub extra_field: Vec<String>,
    pub extra_field_len: String,
    pub extra_index: String,
    pub jwt_payload_without_sha_padding: Vec<String>,
    pub use_extra_field: String,
}

// Struct with all fields converted to U256
#[allow(dead_code)]
#[derive(Debug)]
pub struct JwtConverted {
    pub jwt: Vec<BN254>,
    pub jwt_header_with_separator: Vec<BN254>,
    pub jwt_payload: Vec<BN254>,
    pub public_inputs_hash: BN254,
    pub header_len_with_separator: BN254,
    pub signature: Vec<BN254>,
    pub pubkey_modulus: Vec<BN254>,
    pub aud_field: Vec<BN254>,
    pub aud_field_string_bodies: Vec<BN254>,
    pub aud_field_len: BN254,
    pub aud_index: BN254,
    pub aud_value_index: BN254,
    pub aud_colon_index: BN254,
    pub aud_name: Vec<BN254>,
    pub uid_field: Vec<BN254>,
    pub uid_field_string_bodies: Vec<BN254>,
    pub uid_field_len: BN254,
    pub uid_index: BN254,
    pub uid_name_len: BN254,
    pub uid_value_index: BN254,
    pub uid_value_len: BN254,
    pub uid_colon_index: BN254,
    pub uid_name: Vec<BN254>,
    pub uid_value: Vec<BN254>,
    pub ev_field: Vec<BN254>,
    pub ev_field_len: BN254,
    pub ev_index: BN254,
    pub ev_value_index: BN254,
    pub ev_value_len: BN254,
    pub ev_colon_index: BN254,
    pub ev_name: Vec<BN254>,
    pub ev_value: Vec<BN254>,
    pub iss_field: Vec<BN254>,
    pub iss_field_string_bodies: Vec<BN254>,
    pub iss_field_len: BN254,
    pub iss_index: BN254,
    pub iss_value_index: BN254,
    pub iss_value_len: BN254,
    pub iss_colon_index: BN254,
    pub iss_name: Vec<BN254>,
    pub iss_value: Vec<BN254>,
    pub nonce_field: Vec<BN254>,
    pub nonce_field_string_bodies: Vec<BN254>,
    pub nonce_field_len: BN254,
    pub nonce_index: BN254,
    pub nonce_value_index: BN254,
    pub nonce_value_len: BN254,
    pub nonce_colon_index: BN254,
    pub nonce_name: Vec<BN254>,
    pub nonce_value: Vec<BN254>,
    pub temp_pubkey: Vec<BN254>,
    pub jwt_randomness: BN254,
    pub pepper: BN254,
    pub jwt_num_sha2_blocks: BN254,
    pub iat_field: Vec<BN254>,
    pub iat_field_len: BN254,
    pub iat_index: BN254,
    pub iat_value_index: BN254,
    pub iat_value_len: BN254,
    pub iat_colon_index: BN254,
    pub iat_name: Vec<BN254>,
    pub iat_value: Vec<BN254>,
    pub exp_date: BN254,
    pub exp_delta: BN254,
    pub b64_payload_len: BN254,
    pub jwt_len_bit_encoded: Vec<BN254>,
    pub padding_without_len: Vec<BN254>,
    pub temp_pubkey_len: BN254,
    pub private_aud_value: Vec<BN254>,
    pub override_aud_value: Vec<BN254>,
    pub private_aud_value_len: BN254,
    pub override_aud_value_len: BN254,
    pub use_aud_override: BN254,
    pub extra_field: Vec<BN254>,
    pub extra_field_len: BN254,
    pub extra_index: BN254,
    pub jwt_payload_without_sha_padding: Vec<BN254>,
    pub use_extra_field: BN254,
}

// Function to convert string -> BN254
fn str_to_u256(value: &str) -> Result<BN254, Box<dyn Error>> {
    // First convert string to ethnum::U256, then convert to BN254
    let u256_value = U256::from_str_radix(value, 10)?;
    Ok(BN254::from_u256(u256_value))
}

// Convert Vec<String> -> Vec<BN254>
fn vec_str_to_u256(vec: &[String]) -> Result<Vec<BN254>, Box<dyn Error>> {
    // Use collect::<Result<_, _>>() to handle the Result inside the map
    vec.iter()
        .map(|s| {
            let u256_value = U256::from_str_radix(s, 10)?;
            Ok(BN254::from_u256(u256_value))
        })
        .collect()
}
// Read JSON, parse, and convert everything to U256
pub fn read_json_file(file_path: &str) -> Result<JwtConverted, Box<dyn Error>> {
    let json_data = fs::read_to_string(file_path)?;
    let jwt: Jwt = serde_json::from_str(&json_data)?;

    Ok(JwtConverted {
        jwt: vec_str_to_u256(&jwt.jwt)?,
        jwt_header_with_separator: vec_str_to_u256(&jwt.jwt_header_with_separator)?,
        jwt_payload: vec_str_to_u256(&jwt.jwt_payload)?,
        public_inputs_hash: str_to_u256(&jwt.public_inputs_hash)?,
        header_len_with_separator: str_to_u256(&jwt.header_len_with_separator)?,
        signature: vec_str_to_u256(&jwt.signature)?,
        pubkey_modulus: vec_str_to_u256(&jwt.pubkey_modulus)?,
        aud_field: vec_str_to_u256(&jwt.aud_field)?,
        aud_field_string_bodies: vec_str_to_u256(&jwt.aud_field_string_bodies)?,
        aud_field_len: str_to_u256(&jwt.aud_field_len)?,
        aud_index: str_to_u256(&jwt.aud_index)?,
        aud_value_index: str_to_u256(&jwt.aud_value_index)?,
        aud_colon_index: str_to_u256(&jwt.aud_colon_index)?,
        aud_name: vec_str_to_u256(&jwt.aud_name)?,
        uid_field: vec_str_to_u256(&jwt.uid_field)?,
        uid_field_string_bodies: vec_str_to_u256(&jwt.uid_field_string_bodies)?,
        uid_field_len: str_to_u256(&jwt.uid_field_len)?,
        uid_index: str_to_u256(&jwt.uid_index)?,
        uid_name_len: str_to_u256(&jwt.uid_name_len)?,
        uid_value_index: str_to_u256(&jwt.uid_value_index)?,
        uid_value_len: str_to_u256(&jwt.uid_value_len)?,
        uid_colon_index: str_to_u256(&jwt.uid_colon_index)?,
        uid_name: vec_str_to_u256(&jwt.uid_name)?,
        uid_value: vec_str_to_u256(&jwt.uid_value)?,
        ev_field: vec_str_to_u256(&jwt.ev_field)?,
        ev_field_len: str_to_u256(&jwt.ev_field_len)?,
        ev_index: str_to_u256(&jwt.ev_index)?,
        ev_value_index: str_to_u256(&jwt.ev_value_index)?,
        ev_value_len: str_to_u256(&jwt.ev_value_len)?,
        ev_colon_index: str_to_u256(&jwt.ev_colon_index)?,
        ev_name: vec_str_to_u256(&jwt.ev_name)?,
        ev_value: vec_str_to_u256(&jwt.ev_value)?,
        iss_field: vec_str_to_u256(&jwt.iss_field)?,
        iss_field_string_bodies: vec_str_to_u256(&jwt.iss_field_string_bodies)?,
        iss_field_len: str_to_u256(&jwt.iss_field_len)?,
        iss_index: str_to_u256(&jwt.iss_index)?,
        iss_value_index: str_to_u256(&jwt.iss_value_index)?,
        iss_value_len: str_to_u256(&jwt.iss_value_len)?,
        iss_colon_index: str_to_u256(&jwt.iss_colon_index)?,
        iss_name: vec_str_to_u256(&jwt.iss_name)?,
        iss_value: vec_str_to_u256(&jwt.iss_value)?,
        nonce_field: vec_str_to_u256(&jwt.nonce_field)?,
        nonce_field_string_bodies: vec_str_to_u256(&jwt.nonce_field_string_bodies)?,
        nonce_field_len: str_to_u256(&jwt.nonce_field_len)?,
        nonce_index: str_to_u256(&jwt.nonce_index)?,
        nonce_value_index: str_to_u256(&jwt.nonce_value_index)?,
        nonce_value_len: str_to_u256(&jwt.nonce_value_len)?,
        nonce_colon_index: str_to_u256(&jwt.nonce_colon_index)?,
        nonce_name: vec_str_to_u256(&jwt.nonce_name)?,
        nonce_value: vec_str_to_u256(&jwt.nonce_value)?,
        temp_pubkey: vec_str_to_u256(&jwt.temp_pubkey)?,
        jwt_randomness: str_to_u256(&jwt.jwt_randomness)?,
        pepper: str_to_u256(&jwt.pepper)?,
        jwt_num_sha2_blocks: str_to_u256(&jwt.jwt_num_sha2_blocks)?,
        iat_field: vec_str_to_u256(&jwt.iat_field)?,
        iat_field_len: str_to_u256(&jwt.iat_field_len)?,
        iat_index: str_to_u256(&jwt.iat_index)?,
        iat_value_index: str_to_u256(&jwt.iat_value_index)?,
        iat_value_len: str_to_u256(&jwt.iat_value_len)?,
        iat_colon_index: str_to_u256(&jwt.iat_colon_index)?,
        iat_name: vec_str_to_u256(&jwt.iat_name)?,
        iat_value: vec_str_to_u256(&jwt.iat_value)?,
        exp_date: str_to_u256(&jwt.exp_date)?,
        exp_delta: str_to_u256(&jwt.exp_delta)?,
        b64_payload_len: str_to_u256(&jwt.b64_payload_len)?,
        jwt_len_bit_encoded: vec_str_to_u256(&jwt.jwt_len_bit_encoded)?,
        padding_without_len: vec_str_to_u256(&jwt.padding_without_len)?,
        temp_pubkey_len: str_to_u256(&jwt.temp_pubkey_len)?,
        private_aud_value: vec_str_to_u256(&jwt.private_aud_value)?,
        override_aud_value: vec_str_to_u256(&jwt.override_aud_value)?,
        private_aud_value_len: str_to_u256(&jwt.private_aud_value_len)?,
        override_aud_value_len: str_to_u256(&jwt.override_aud_value_len)?,
        use_aud_override: str_to_u256(&jwt.use_aud_override)?,
        extra_field: vec_str_to_u256(&jwt.extra_field)?,
        extra_field_len: str_to_u256(&jwt.extra_field_len)?,
        extra_index: str_to_u256(&jwt.extra_index)?,
        jwt_payload_without_sha_padding: vec_str_to_u256(&jwt.jwt_payload_without_sha_padding)?,
        use_extra_field: str_to_u256(&jwt.use_extra_field)?,
    })
}