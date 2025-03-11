mod json_utils;
use json_utils::{read_json_file, Jwt};
use std::io::{self, Write};
use expander_compiler::frontend::*;
use expander_compiler::frontend::extra::*;


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

fn main() {
println!("Inputs Reading...");
let file_name = "input.json";
// let compile_result=compile(&JWTVerification::default(),
// CompileOptions::default()).unwrap();

let jwt:Jwt = read_json_file(file_name).unwrap();


// debug_eval::<BN254Config,_,_,_>(&JWTVerification::default(),&assignment, EmptyHintCaller::new())

}

