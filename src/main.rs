use anyhow;
use reqwest;
use reqwest::header;
use reqwest::header::HeaderMap;
use serde_json::json;

mod cfg;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    completions().await?;

    Ok(())
}

async fn completions() -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let payload = json!({
        "model":"gpt-4-0613",
        "messages":[{"role":"system","content":"You are ChatGPT, a large language model trained by OpenAI. Answer as concisely as possible. Knowledge cutoff: 2021-09-01. Current date: 2023-07-24"},{"role":"user","content":"btc"}],
        "stream":true
    });

    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        format!("Bearer  {}", cfg::API_KEY).parse().unwrap(),
    );

    let rsp = client
        .post(cfg::URL)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;
    let content = rsp.text().await?;
    // println!("rsp {:#?}", rsp);
    println!("content {:#?}", content);
    Ok(())
}
