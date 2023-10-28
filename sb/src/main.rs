use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let response = reqwest::get("https://www.google.com").await?;
    println!("Status: {}", response.status());
    println!("Headers:\n{:?}", response.headers());
    println!("Body:\n{}", response.text().await?);
    Ok(())
}