use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use js_sys::Promise;

use crate::profile::Profile;
use crate::api::Api;


#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    public: String,
    group: String,
    key: String,
    version: u64,
    data: String,
    signature: String,
}


#[wasm_bindgen]
impl Block {
    pub fn new(publicKey: &str, group: &str, key: &str) -> Self {
        Self {
            public: publicKey.to_string(),
            group: group.to_string(),
            key: key.to_string(),
            version: 0,
            data: "".to_string(),
            signature: "".to_string(),
        }
    }

    pub fn fromBlockJson(blockJson: &JsValue) -> Self {
        // Create Block object
        let block: Self = blockJson.into_serde().unwrap();

        // Check signature
        let iscorrect = Profile::checkSignature(
            &block.public, &block.group, &block.key,
            block.version, &block.data, &block.signature
        );
        assert!(iscorrect);

        // Return
        block
    }

    pub fn publicKey(&self) -> String {
        self.public.clone()
    }

    pub fn group(&self) -> String {
        self.group.clone()
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn data(&self) -> String {
        self.data.clone()
    }

    pub fn signature(&self) -> String {
        self.signature.clone()
    }

    pub fn incVersion(&mut self) {
        self.version += 1;
        self.clearSignature();
    }

    pub fn setData(&mut self, data: &str) {
        self.data = data.to_string();
        self.clearSignature();
    }

    pub fn isSigned(&self) -> bool {
        !self.signature.is_empty()
    }

    pub fn sign(&mut self, profile: &Profile) {
        assert_eq!(self.public, profile.publicKey());
        self.signature = profile.buildSignature(
            &self.group, &self.key, self.version, &self.data
        );
    }

    pub fn clearSignature(&mut self) {
        self.signature = "".to_string();
    }

    pub fn update(&self, api: &Api) -> Promise {
        api.postData(
            &self.public, &self.group, &self.key,
            self.version, &self.data, &self.signature
        )
    }

    pub fn save(&mut self, api: &Api, profile: &Profile) -> Promise {
        self.incVersion();
        self.sign(profile);
        self.update(api)
    }
}
