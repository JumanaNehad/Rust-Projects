//to run : cargo run
#![deny(clippy::all)]

//reqest by blocking not by async await
use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    //mn reqwest making a request
    let client = Client::new();

    //create a user
    let user = "testuser".to_string();
    // speeling different bec it's written like that in the url
    let passwd: Option<String> = None;

    //use client to make request and put that in response as we will recive response
    //GET request to a URL with HTTP basic authentication
    let response = client
        .get("http://httpbin.org/get")
        .basic_auth(user, passwd) //dd HTTP basic authentication to the request
        .send();

    println!("{:?}", response);
    Ok(())
}
