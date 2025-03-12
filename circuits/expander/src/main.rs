mod json_utils;
use json_utils::{read_json_file,JwtConverted};
use std::io::{self, Write};
use expander_compiler::frontend::*;
use expander_compiler::frontend::extra::*;
use std::convert::TryInto;
use std::error::Error;

declare_circuit!(JWTVerification {
    jwt:[Variable;192*8],
    jwt_header_with_separator:[Variable;300],
    jwt_payload:[Variable; 192*8-64],
    header_len_with_separator:Variable,
    b64_payload_len:Variable,
    jwt_num_sha2_blocks:Variable,
    jwt_len_bit_encoded:[Variable;8],
    padding_without_len:[Variable;64],
    signature:[Variable;32],
    pubkey_modulus:[Variable;32],
    jwt_payload_without_sha_padding:[Variable;192 * 8 - 64],
    aud_field:[Variable;140],
    aud_field_string_bodies:[Variable;140],
    aud_field_len:Variable,
    aud_index:Variable,
    aud_value_index:Variable,
    aud_colon_index:Variable,
    aud_name:[Variable;40],
    use_aud_override:Variable,
    private_aud_value:[Variable;120],
    override_aud_value:[Variable;120],
    private_aud_value_len:Variable,
    override_aud_value_len:Variable,
    uid_field:[Variable;350],
    uid_field_string_bodies:[Variable;350],
    uid_field_len:Variable,
    uid_index:Variable,
    uid_name_len:Variable,
    uid_value_index:Variable,
    uid_value_len:Variable,
    uid_colon_index:Variable,
    uid_name:[Variable;30],
    uid_value:[Variable;330],
    extra_field:[Variable;350],
    extra_field_len:Variable,
    extra_index:Variable,
    use_extra_field:Variable,
    ev_field:[Variable;350],
    ev_field_len:Variable,
    ev_index:Variable,
    ev_value_index:Variable,
    ev_value_len:Variable,
    ev_colon_index:Variable,
    ev_name:[Variable;20],
    ev_value:[Variable;10],
    iss_field:[Variable;140],
    iss_field_string_bodies:[Variable;140],
    iss_field_len:Variable,
    iss_index:Variable,
    iss_value_index:Variable,
    iss_value_len:Variable,
    iss_colon_index:Variable,
    iss_name:[Variable;40],
    iss_value:[Variable;120],
    iat_field:[Variable;50],
    iat_field_len:Variable,
    iat_index:Variable,
    iat_value_index:Variable,
    iat_value_len:Variable,
    iat_colon_index:Variable,
    iat_name:[Variable;10],
    iat_value:[Variable;45],
    exp_date:Variable,
    exp_delta:Variable,
    nonce_field:[Variable;105],
    nonce_field_string_bodies:[Variable; 105],
     nonce_field_len:Variable,
     nonce_index:Variable,
     nonce_value_index:Variable,
     nonce_value_len:Variable,
     nonce_colon_index:Variable,
     nonce_name:[Variable;10],
     nonce_value:[Variable;100],
     temp_pubkey:[Variable;3],
     temp_pubkey_len:Variable,
     jwt_randomness:Variable,
     pepper:Variable,
     public_inputs_hash:PublicVariable
});

impl Define<BN254Config> for JWTVerification<Variable>{
    fn define<Builder: RootAPI<BN254Config>>(&self,api: &mut Builder)
    {
        
       
    }
}

// Function to convert Vec<BN254> to a fixed-size array
fn vec_to_array<T: Copy, const N: usize>(vec: Vec<T>) -> Result<[T; N], Box<dyn Error>> {
    if vec.len() != N {
        return Err(format!("Expected vector of length {}, but got {}", N, vec.len()).into());
    }
    
    let mut arr = [vec[0]; N]; // Initialize with the first element
    for (i, item) in vec.iter().enumerate() {
        arr[i] = *item;
    }
    
    Ok(arr)
}

fn main()-> Result<(), Box<dyn std::error::Error>> {
println!("Inputs Reading...");
let file_name = "input.json";
let jwt_converted:JwtConverted = read_json_file(file_name).unwrap();

// println!("{:?}", jwt.jwt);


let mut assignment = JWTVerification::<BN254>{
    jwt: vec_to_array(jwt_converted.jwt)?,
    jwt_header_with_separator: vec_to_array(jwt_converted.jwt_header_with_separator)?,
    jwt_payload: vec_to_array(jwt_converted.jwt_payload)?,
    header_len_with_separator: jwt_converted.header_len_with_separator,
    b64_payload_len: jwt_converted.b64_payload_len,
    jwt_num_sha2_blocks: jwt_converted.jwt_num_sha2_blocks,
    jwt_len_bit_encoded: vec_to_array(jwt_converted.jwt_len_bit_encoded)?,
    padding_without_len: vec_to_array(jwt_converted.padding_without_len)?,
    signature: vec_to_array(jwt_converted.signature)?,
    pubkey_modulus: vec_to_array(jwt_converted.pubkey_modulus)?,
    jwt_payload_without_sha_padding: vec_to_array(jwt_converted.jwt_payload_without_sha_padding)?,
    aud_field: vec_to_array(jwt_converted.aud_field)?,
    aud_field_string_bodies: vec_to_array(jwt_converted.aud_field_string_bodies)?,
    aud_field_len: jwt_converted.aud_field_len,
    aud_index: jwt_converted.aud_index,
    aud_value_index: jwt_converted.aud_value_index,
    aud_colon_index: jwt_converted.aud_colon_index,
    aud_name: vec_to_array(jwt_converted.aud_name)?,
    use_aud_override: jwt_converted.use_aud_override,
    private_aud_value: vec_to_array(jwt_converted.private_aud_value)?,
    override_aud_value: vec_to_array(jwt_converted.override_aud_value)?,
    private_aud_value_len: jwt_converted.private_aud_value_len,
    override_aud_value_len: jwt_converted.override_aud_value_len,
    uid_field: vec_to_array(jwt_converted.uid_field)?,
    uid_field_string_bodies: vec_to_array(jwt_converted.uid_field_string_bodies)?,
    uid_field_len: jwt_converted.uid_field_len,
    uid_index: jwt_converted.uid_index,
    uid_name_len: jwt_converted.uid_name_len,
    uid_value_index: jwt_converted.uid_value_index,
    uid_value_len: jwt_converted.uid_value_len,
    uid_colon_index: jwt_converted.uid_colon_index,
    uid_name: vec_to_array(jwt_converted.uid_name)?,
    uid_value: vec_to_array(jwt_converted.uid_value)?,
    extra_field: vec_to_array(jwt_converted.extra_field)?,
    extra_field_len: jwt_converted.extra_field_len,
    extra_index: jwt_converted.extra_index,
    use_extra_field: jwt_converted.use_extra_field,
    ev_field: vec_to_array(jwt_converted.ev_field)?,
    ev_field_len: jwt_converted.ev_field_len,
    ev_index: jwt_converted.ev_index,
    ev_value_index: jwt_converted.ev_value_index,
    ev_value_len: jwt_converted.ev_value_len,
    ev_colon_index: jwt_converted.ev_colon_index,
    ev_name: vec_to_array(jwt_converted.ev_name)?,
    ev_value: vec_to_array(jwt_converted.ev_value)?,
    iss_field: vec_to_array(jwt_converted.iss_field)?,
    iss_field_string_bodies: vec_to_array(jwt_converted.iss_field_string_bodies)?,
    iss_field_len: jwt_converted.iss_field_len,
    iss_index: jwt_converted.iss_index,
    iss_value_index: jwt_converted.iss_value_index,
    iss_value_len: jwt_converted.iss_value_len,
    iss_colon_index: jwt_converted.iss_colon_index,
    iss_name: vec_to_array(jwt_converted.iss_name)?,
    iss_value: vec_to_array(jwt_converted.iss_value)?,
    iat_field: vec_to_array(jwt_converted.iat_field)?,
    iat_field_len: jwt_converted.iat_field_len,
    iat_index: jwt_converted.iat_index,
    iat_value_index: jwt_converted.iat_value_index,
    iat_value_len: jwt_converted.iat_value_len,
    iat_colon_index: jwt_converted.iat_colon_index,
    iat_name: vec_to_array(jwt_converted.iat_name)?,
    iat_value: vec_to_array(jwt_converted.iat_value)?,
    exp_date: jwt_converted.exp_date,
    exp_delta: jwt_converted.exp_delta,
    nonce_field: vec_to_array(jwt_converted.nonce_field)?,
    nonce_field_string_bodies: vec_to_array(jwt_converted.nonce_field_string_bodies)?,
    nonce_field_len: jwt_converted.nonce_field_len,
    nonce_index: jwt_converted.nonce_index,
    nonce_value_index: jwt_converted.nonce_value_index,
    nonce_value_len: jwt_converted.nonce_value_len,
    nonce_colon_index: jwt_converted.nonce_colon_index,
    nonce_name: vec_to_array(jwt_converted.nonce_name)?,
    nonce_value: vec_to_array(jwt_converted.nonce_value)?,
    temp_pubkey: vec_to_array(jwt_converted.temp_pubkey)?,
    temp_pubkey_len: jwt_converted.temp_pubkey_len,
    jwt_randomness: jwt_converted.jwt_randomness,
    pepper: jwt_converted.pepper,
    public_inputs_hash: jwt_converted.public_inputs_hash
};
// let compile_result=compile(&JWTVerification::default(),
// CompileOptions::default()).unwrap();

// debug_eval::<BN254Config,_,_,_>(&JWTVerification::default(),&assignment, EmptyHintCaller::new())
Ok(())

}

