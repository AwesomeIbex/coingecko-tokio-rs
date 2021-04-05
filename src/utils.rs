use crate::Error;
use serde::de::DeserializeOwned;

pub async fn get_json<T: DeserializeOwned>(client: &reqwest::Client, uri: &str) -> Result<T, Error> {
    let request: T = client.get(uri)
        .header("content-type", "application/javascript")
        .send()
        .await?
        .json()
        .await?;

    Ok(request)
}
