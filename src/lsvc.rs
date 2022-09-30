use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::utils::get_local_storage;
use crate::block::Block;


/// Prefix for local storage records.
const LSVC_PREFIX_DEFAULT: &str = "hslsvc";


/// LSVC (local storage version checker) is an object that checks and manages
/// block versions in the remove Hashstorage instance and the ones saved in 
/// local storage.
#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct LSVC {
    prefix: String,
}


#[wasm_bindgen]
impl LSVC {
    /// Creates a LSVC instance.
    pub fn new() -> Self {
        Self {
            prefix: LSVC_PREFIX_DEFAULT.to_string(),
        }
    }

    /// Saves the version to localstorage.
    pub fn saveVersion(&self, block: &Block) {
        let ls_key = self._get_ls_key(block);
        let storage = get_local_storage();
        storage.set_item(&ls_key, &block.version().to_string()).unwrap();
    }

    /// Checks the version in localstorage. The method returns `false` if the 
    /// version exists in local storage and it is higher than the given one.
    /// Else it returns `true`.
    pub fn checkVersion(&self, block: &Block) -> bool {
        let ls_key = self._get_ls_key(block);
        let storage = get_local_storage();
        match storage.get_item(&ls_key).unwrap() {
            Some(ls_version) => {
                block.version() >= ls_version.parse::<u64>().unwrap()
            },
            None => true
        }
    }

    fn _get_ls_key(&self, block: &Block) -> String {
        let hash = Self::_get_hash(&block.publicKey(), &block.group(), &block.key());
        format!("{}-{}", self.prefix, hash)
    }

    fn _get_hash(publicKey: &str, group: &str, key: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        publicKey.hash(&mut hasher);
        group.hash(&mut hasher);
        key.hash(&mut hasher);
        hasher.finish()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hash() {
        let hash = LSVC::_get_hash(
            "F97CF0EA9BA1C36BE29045A14AAC32ED9ECD8D67A9D6823D623E161B2600ED3B4D3FA95A1580FED6068BD67013C990524DCCE132350EAC38948E3E15BC3E1E60",
            "mygroup", 
            "mykey"
        );
        assert_eq!(hash, 9969592579325465869);
    }

    #[test]
    fn test_get_ls_key() {
        let block = Block::new(
            "F97CF0EA9BA1C36BE29045A14AAC32ED9ECD8D67A9D6823D623E161B2600ED3B4D3FA95A1580FED6068BD67013C990524DCCE132350EAC38948E3E15BC3E1E60",
            "mygroup", 
            "mykey"
        );
        let lsvc = LSVC::new();
        let ls_key = lsvc._get_ls_key(&block);
        assert_eq!(ls_key, "hslsvc-9969592579325465869");
    }
}
