#![deny(clippy::all)]
//run: cargo watch or cargo run
use error_chain::error_chain;
use std::io::Read;

//optional instead we can use : , Box<dyn std::error::Error>
//used before ? for complex error
//handle er
error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    //make blocking get request
    //making an HTTP GET request to the URL
    //reqwest's blocking API, which means it will block the thread it's running on until the request is completed and a response is returned or an error occurs.
    //The ? operator at the end of the request is used for error handling.
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    //creates a new, empty instance of a String
    //This variable is intended to hold the data that will be read from the response
    let mut body = String::new();
    //The read_to_string: method is convenient for when you need to handle all the data as a single string, but it requires that the data be valid UTF-8, as it decodes the data into a Rust String.
    //read all bytes from res until EOF (end of file) and appends them to body as a String
    res.read_to_string(&mut body)?;
    println!("Status: {}", res.status());
    println!("Header: {:#?}", res.headers());
    println!("body: \n{}", body);

    //OR:
    // if response.status().is_success() {
    //     let body = response.text()?; // Read the response text
    //     println!("Response Text: {}", body);
    // } else {
    //     println!("Failed to make request: {}", response.status());
    // }

    Ok(())
}
