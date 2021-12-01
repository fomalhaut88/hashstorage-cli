use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use js_sys::Promise;

use crate::utils::http_request_json;


#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct Api {
    root: String,
}


#[wasm_bindgen]
impl Api {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }

    pub fn root(&self) -> String {
        self.root.clone()
    }

    pub fn getVersion(&self) -> Promise {
        let url = format!("{}/version", self.root);
        future_to_promise(async move {
            http_request_json(&url, "GET", None).await
        })
    }

    pub fn getGroups(&self, publicKey: &str) -> Promise {
        let url = format!("{}/groups/{}", self.root, publicKey);
        future_to_promise(async move {
            http_request_json(&url, "GET", None).await
        })
    }

    pub fn getKeys(&self, publicKey: &str, group: &str) -> Promise {
        let url = format!("{}/keys/{}/{}", self.root, publicKey, group);
        future_to_promise(async move {
            http_request_json(&url, "GET", None).await
        })
    }

    pub fn getInfo(&self, publicKey: &str, group: &str, key: &str) -> Promise {
        let url = format!(
            "{}/info/{}/{}/{}", self.root, publicKey, group, key
        );
        future_to_promise(async move {
            http_request_json(&url, "GET", None).await
        })
    }

    pub fn getData(&self, publicKey: &str, group: &str, key: &str) -> Promise {
        let url = format!(
            "{}/data/{}/{}/{}", self.root, publicKey, group, key
        );
        future_to_promise(async move {
            http_request_json(&url, "GET", None).await
        })
    }

    pub fn postData(&self, publicKey: &str, group: &str, key: &str,
                    version: u64, data: &str, signature: &str) -> Promise {
        let url = format!(
            "{}/data/{}/{}/{}", self.root, publicKey, group, key
        );
        let body = Self::_prepareInputBody(version, data, signature);

        future_to_promise(async move {
            http_request_json(&url, "POST", Some(body)).await
        })
    }

    fn _prepareInputBody(
                version: u64, data: &str, signature: &str
            ) -> JsValue {
        let input_json = InputJson {
            version,
            data: data.to_string(),
            signature: signature.to_string(),
        };
        JsValue::from_serde(&input_json).unwrap()
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct InputJson {
    version: u64,
    data: String,
    signature: String,
}
