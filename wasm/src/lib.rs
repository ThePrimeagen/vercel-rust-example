mod auth;
use libsql_client::{new_client_from_config, Config, DatabaseClient, ResultSet, Row, Value};
use url::Url;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn xor(a: u32, b: u32) -> Result<u32, JsValue> {
    return Ok(a ^ b);
}

#[wasm_bindgen]
pub async fn async_test(_i: usize) -> Result<String, JsError> {
    let url: Url = "https://theprimeagen-theprimeagen.turso.io"
        .try_into()
        .expect("to be a url");

    web_sys::console::log_1(&JsValue::from(format!("URL: {}", url)));

    let client = match new_client_from_config(Config {
        url,
        auth_token: Some(auth::TOKEN.to_string()),
    })
    .await {
        Ok(client) => client,
        Err(e) => {
            return Ok(format!("UNABLE TO GET CLIENT: {}", e));
        }
    };

    let results = match client.execute("select count from counter").await {
        Ok(results) => results,
        Err(e) => {
            return Ok(format!("UNABLE TO GET CLIENT: {}", e));
        }
    };

    let items: &Value = results.rows.get(0).expect("one result").values.get(0).expect("one value");
    let count = match items {
        Value::Integer { value } => value,
        _ => {
            return Ok("UNABLE TO GET COUNT".into());
        }
    };
    _ = client.execute(format!("UPDATE counter
SET count = {}
WHERE
    count = {}", count + 1, count)).await;

    return Ok(format!("your counter is {}", count).into());
}
