use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Storage};


pub async fn http_request_json(
            url: &str, method: &str, body: Option<JsValue>
        ) -> Result<JsValue, JsValue> {
    // Prepare request opts
    let mut opts = RequestInit::new();
    opts.method(method);
    opts.mode(RequestMode::Cors);

    // Set JSON body
    if body.is_some() {
        opts.body(
            Some(&js_sys::JSON::stringify(&body.as_ref().unwrap()).unwrap())
        );
    }

    // Prepare request
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/json")?;

    // Set Content-Type to application/json
    if body.is_some() {
        request.headers().set("Content-Type", "application/json")?;
    }

    // Make request
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(
        window.fetch_with_request(&request)
    ).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    // Get JSON data
    let json = if resp.status() == 200 {
        JsFuture::from(resp.json()?).await?
    } else {
        JsValue::null()
    };

    Ok(json)
}


pub fn get_local_storage() -> Storage {
    let window = web_sys::window().unwrap();
    window.local_storage().unwrap().unwrap()
}
