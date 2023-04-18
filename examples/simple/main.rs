use serde::{Deserialize, Serialize};
use serde_crypt_macro::serde_crypt_gen;

#[serde_crypt_gen]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Test {
    #[serde(with = "serde_crypt")]
    pub field: Vec<u8>,
}

fn main() {}
