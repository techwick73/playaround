use std::io::{Error, ErrorKind};

#[allow(dead_code)]
async fn my_async_call(url: &str) -> Result<serde_json::Value, Error> {
    // Original way
    // let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    // Ok(response)

    let response = reqwest::get(url)
        .await
        .map_err(|_| Error::new(ErrorKind::Other, "Could not retrieve response"))?;

    let json_response = response
        .json::<serde_json::Value>()
        .await
        .map_err(|_| Error::new(ErrorKind::Other, "Could not decode to json"))?;

    Ok(json_response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_async_fn() {
        let api_url = "https://cat-fact.herokuapp.com/facts/";
        let my_res = my_async_call(api_url).await;

        match my_res {
            Ok(r) => {
                dbg!(r)
            }
            Err(_) => {
                panic!("Failed to make a request")
            }
        };
    }
}
