use wasm_bindgen::prelude::*;
use gloo_utils::format::JsValueSerdeExt;
use serde::{Serialize, Deserialize};

mod solana;

// #[wasm_bindgen]
// extern {
//     pub fn alert(s: &str);
// }

#[derive(Serialize, Deserialize)]
struct Result {
    error: String,
    encoding: String,
    program: String,
}

#[wasm_bindgen]
pub fn foo(address: &str) -> JsValue {
    let valid = solana::address::validate(address);

    let error = match valid {
        Ok(_) => String::new(),
        Err(err) => err.to_string(),
    };

    let encoding = match valid {
        Ok(enc) => match enc {
            solana::address::Encoding::Base58 => "Base58".to_string(),
            solana::address::Encoding::Base64 => "Base64".to_string(),
        },
        Err(_) => String::new(),
    };

    let result = Result {
        error: error,
        encoding: encoding,
        program: String::new(),
    };
    return JsValue::from_serde(&result).unwrap();
}