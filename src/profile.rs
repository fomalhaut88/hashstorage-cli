use std::convert::TryInto;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use js_sys::Promise;
use sha2::{Sha256, Digest};
use bigi::Bigi;
use bigi_ecc::Point;
use bigi_ecc::schemas::load_secp256k1;
use hashstorage_utils::convert::*;
use hashstorage_utils::crypto::{build_signature, check_signature};

use crate::utils::get_local_storage;
use crate::api::Api;


/// A profile object that contains the key pair.
#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    publicKey: String,
    privateKey: String,
}


#[wasm_bindgen]
impl Profile {
    /// Creates a profile object by given `appid`, `username` and `password`.
    pub fn new(appid: &str, username: &str, password: &str) -> Self {
        let hash = Self::_hashInit(appid, username, password);
        Self::_fromBytes32(&hash)
    }

    /// Checks whether the profile exists in local storage.
    pub fn exists() -> bool {
        let storage = get_local_storage();
        let publicKey = storage.get_item("hsPublicKey").unwrap();
        let privateKey = storage.get_item("hsPrivateKey").unwrap();
        publicKey.is_some() && privateKey.is_some()
    }

    /// Loads the profile object from local storage.
    pub fn load() -> Self {
        let storage = get_local_storage();
        Self {
            publicKey: storage.get_item("hsPublicKey").unwrap().unwrap(),
            privateKey: storage.get_item("hsPrivateKey").unwrap().unwrap(),
        }
    }

    /// Saves to local storage.
    pub fn save(&self) {
        let storage = get_local_storage();
        storage.set_item("hsPublicKey", &self.publicKey).unwrap();
        storage.set_item("hsPrivateKey", &self.privateKey).unwrap();
    }

    /// Clears the saved profile objects in local storage.
    pub fn clear(&self) {
        let storage = get_local_storage();
        storage.remove_item("hsPublicKey").unwrap();
        storage.remove_item("hsPrivateKey").unwrap();
    }

    /// Gets public key.
    pub fn publicKey(&self) -> String {
        self.publicKey.clone()
    }

    /// Gets private key.
    pub fn privateKey(&self) -> String {
        self.privateKey.clone()
    }

    /// Gets groups from hashstorage backend.
    pub fn getGroups(&self, api: &Api) -> Promise {
        api.getGroups(&self.publicKey)
    }

    /// Gets keys from hashstorage backend.
    pub fn getKeys(&self, api: &Api, group: &str) -> Promise {
        api.getKeys(&self.publicKey, &group)
    }

    /// Gets block data from hashstorage backend.
    pub fn getBlockJson(&self, api: &Api, group: &str, key: &str) -> Promise {
        api.getData(&self.publicKey, group, key)
    }

    /// Checks whether the key pair is valid.
    pub fn check(&self) -> bool {
        let private = private_key_from_bytes(
            &hex_to_bytes::<32>(&self.privateKey)
        );
        let public = public_key_from_bytes(
            &hex_to_bytes::<64>(&self.publicKey)
        );
        let schema = load_secp256k1();
        schema.get_point(&private) == public
    }

    /// Builds a signature for the given data.
    pub fn buildSignature(
                &self, group: &str, key: &str, version: u64, data: &str
            ) -> String {
        let mut rng = rand::thread_rng();
        let schema = load_secp256k1();
        let signature = build_signature(
            &mut rng,
            &schema,
            &hex_to_bytes::<32>(&self.privateKey),
            &str_to_bytes_sized::<32>(&group),
            &str_to_bytes_sized::<32>(&key),
            version,
            &data.as_bytes()
        );
        hex_from_bytes(&signature)
    }

    /// Checks the signature for the data.
    pub fn checkSignature(
                publicKey: &str, group: &str, key: &str,
                version: u64, data: &str, signature: &str
            ) -> bool {
        let schema = load_secp256k1();
        check_signature(
            &schema,
            &hex_to_bytes(&signature),
            &hex_to_bytes(&publicKey),
            &str_to_bytes_sized(&group),
            &str_to_bytes_sized(&key),
            version,
            &data.as_bytes()
        )
    }

    fn _fromBytes32(bytes: &[u8; 32]) -> Self {
        let (privateKey, publicKey) = Self::_buildPair(&bytes);
        Self {
            publicKey: hex_from_bytes(&public_key_to_bytes(&publicKey)),
            privateKey: hex_from_bytes(&private_key_to_bytes(&privateKey)),
        }
    }

    fn _hashInit(appid: &str, username: &str, password: &str) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.reset();
        hasher.update(appid.as_bytes());
        hasher.update(b":");
        hasher.update(username.as_bytes());
        hasher.update(b":");
        hasher.update(password.as_bytes());
        hasher.finalize().try_into().unwrap()
    }

    fn _buildPair(hash: &[u8; 32]) -> (Bigi<4>, Point<4>) {
        let hashBigi = Bigi::<4>::from_bytes(hash);
        let schema = load_secp256k1();
        let privateKey = schema.get_point(&hashBigi).x;
        let publicKey = schema.get_point(&privateKey);
        (privateKey, publicKey)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let profile = Profile::new("appidstring", "alex", "Qwerty123");
        assert!(profile.check());
    }

    #[test]
    fn test_signature() {
        let group = "mygroup";
        let key = "mykey";
        let version = 1;
        let data = "yes";

        let profile = Profile::new("appidstring", "alex", "Qwerty123");
        let signature = profile.buildSignature(&group, &key, version, &data);
        let result = Profile::checkSignature(
            &profile.publicKey(), &group, &key, version, &data, &signature
        );
        assert!(result);
    }
}
