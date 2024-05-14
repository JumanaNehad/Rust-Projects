#![deny(clippy::all)]
use reqwest::header::USER_AGENT; //he USER_AGENT constant is used to represent the "User-Agent" header in HTTP requests.
use reqwest::Error; //to encapsulate all possible types of errors that can occur while making HTTP requests, such as network errors, decoding errors
use serde::Deserialize; //The Deserialize trait is used to define how a data structure can be deserialized from a format (like JSON) into a Rust data structure.

#[derive(Deserialize, Debug)]
struct User {
    //what we willl use from api just define it here 
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(),Error>{
    //This line builds a URL that is used to query the GitHub API for the stargazers of the "rust-cookbook" repository in the "rust-lang-nursery" GitHub organization.
let request_url= format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
owner="rust-lang-nursery", //org
repo="rust-cookbook"); //repo

println!("{}",request_url);

// Using a Client is more efficient than creating requests directly when making multiple requests because it reuses underlying TCP connections.
let client = reqwest::Client::new();
//send a GET request to a specified URL, and then await the response.
let response= client.
get(&request_url) //you're retrieving data 
.header(USER_AGENT,"rust web-api-client demo")//HTTP header to the request
.send()
.await?;

//This indicates that the expected type of the parsed JSON data is a vector of User structs.
let users:Vec<User>= response.json().await?;//.json() method is an asynchronous operation provided by reqwest that tries to parse the JSON content of the response body into a Rust data structure. 
//without debug trait this will give error
println!("{:?}",users);

    Ok(())
}
