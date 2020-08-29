#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://openccpm.com/redmine/projects.json";
    println!("call {}", url);
    let res = reqwest::get(url).await?;
    let body = res.text().await?;
    println!("response is {}\n", body);
    Ok(())
}
