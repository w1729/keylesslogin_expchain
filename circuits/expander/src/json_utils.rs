use serde::Deserialize;
use std::fs;
use std::error::Error;


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


pub fn read_json_file(file_path: &str) -> Result<Jwt, Box<dyn Error>> {
    // Read the JSON file into a string
    let json_data = fs::read_to_string(file_path)?;

    // Parse the JSON string into the Person struct
    let jwt: Jwt = serde_json::from_str(&json_data)?;

    Ok(jwt)
}

