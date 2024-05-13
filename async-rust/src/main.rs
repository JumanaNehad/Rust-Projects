//I could have used error-chain 
//run : cargo watch or cargo run
//same as the get request by blocking but this is with async and await
#![deny(clippy::all)]
use std::error::Error;

#[tokio::main]
async fn main() ->Result<(),Box<dyn Error>> {
let  res =reqwest::get("http://httpbin.org/get").await?;

println!("Status:{} ",res.status());
println!("Headers: {:#?}",res.headers());
let body = res.text().await?;
println!("Body:/n{}",body);
    Ok(())
}
