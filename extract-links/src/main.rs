#![deny(clippy::all)]
//extract links of HTML document of any website
use error_chain::error_chain;
use select::document::Document; // Allows for parsing and querying HTML/XML documents
                                //can select things also from the page
use select::predicate::Name; //// Facilitates selecting elements by tag name

//instead of Box error
error_chain! {
    //handle anything related with io
    foreign_links {
        //two types of error
       ReqError(reqwest::Error); // For handling HTTP request errors
       IoError(std::io::Error); // For handling I/O errors, like file operations
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    //without client 
    //text: to extract the body of the response as a string. 
    let res = reqwest::get("https://www.rust-lang.org/en-US/").await?.text().await?;

    Document::from(res.as_str()) //Converts the response string (res) into a string slice.
    .find(Name("a")) //find all elements in the Document that match the tag name <a>
    .filter_map(|n|n.attr("href")) //This method filters and maps the iterator at the same time. It iterates over all the anchor tags found in the previous step.//For each anchor tag (n), it attempts to retrieve the value of the href attribute. //If attr("href") finds an href attribute, it will be passed through; otherwise, the anchor tag will be filtered out
    .for_each(|x| println!("{}",x)); //.for_each(...): Applies a function to every item in the iterator. In this case, it takes each href value found 


    Ok(())
}
