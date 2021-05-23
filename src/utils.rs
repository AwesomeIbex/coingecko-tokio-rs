use crate::Error;
use serde::de::DeserializeOwned;

pub async fn get_json<T: DeserializeOwned>(client: &reqwest::Client, uri: &str) -> Result<T, Error> {
    let response = client.get(uri)
        .header("content-type", "application/javascript")
        .send()
        .await?;
    let text = response.text().await?;
    log::trace!("response: {:?}", text);
    let request: T = serde_json::from_str(&text)?;

    Ok(request)
}
