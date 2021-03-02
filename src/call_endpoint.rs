use serde_json::Value;

pub async fn get_inspiring_quote() -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    // get a response, parse with a Generic type
    let resp: Value = client
        .get("https://quotes.rest/qod?category=inspire&language=en")
        .send()
        .await?
        .json::<Value>()
        .await?;
    // y'all be good bois and define a strong type for the response
    let quote = &resp["contents"]["quotes"][0]["quote"];
    Ok(quote.as_str().unwrap().to_string())
}
