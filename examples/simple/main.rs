use serde::{Deserialize, Serialize};
use serde_crypt_macro::serde_crypt_gen;

use serde_crypt;

#[serde_crypt_gen]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct TestWrapper {
    #[serde_crypt_types(TestEncrypted, TestDecrypted)]
    pub keys: _,
    pub activator: String,
}

#[serde_crypt_gen]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Test {
    #[serde(with = "serde_crypt")]
    pub field: Vec<u8>,
    pub other: String,
}

fn main() {}
