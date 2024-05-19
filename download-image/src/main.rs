#![deny(clippy::all)]

use error_chain::error_chain;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;


//instead of Box error
error_chain! {
    //handle anything related with io
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
#[tokio::main]
async fn main() ->Result<()>{

    //Create tem directory 
//prefix("example") is a method call on the Builder instance. It sets the prefix of the temporary directory's name. 
//tempdir() is a method on the Builder instance that actually creates the temporary directory. 
let temp_dir = Builder::new().prefix("example").tempdir()?;
let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
//we dont make client 3alatoln req::get
let response= reqwest::get(target).await?;

let mut dest ={
    let fname= response
    .url()//This line retrieves the URL from which the response was fetched. 
    .path_segments() //break the path part of the URL into segments. A URL path is typically structured as /segment1/segment2/segment3, and path_segments() will turn this into an iterator over the segments ["segment1", "segment2", "segment3"].
    .and_then(|segments|segments.last())//and_then is used to handle the Option returned by path_segments(), segments.last() is called to get the last segment of the path, which is often used as the filename in URLs.
    .and_then(|name| if name.is_empty(){None} else{Some(name)}) //This part checks if the last segment (presumably the filename) is empty. (empty ya3ni akhro /)
    .unwrap_or("tmp.bin");//If any of the previous steps result in None (either because there are no path segments or the last segment is empty), unwrap_or provides a default filename, "tmp.bin"

    println!("Name of the file to download: {}",fname);
    //The path() method returns the path to the temporary directory.
    //fname is a variable that holds the filename. By using join, you (concate)append the filename to the path of the temporary directory, effectively creating a full path that points to a new file location inside the temporary directory.
    let file_path = temp_dir.path().join(fname);
    println!{"Will be located under: {:?}", file_path};

     // Prevent the temporary directory from being deleted automatically
     temp_dir.into_path();
     
    //create a new file at the path specified by fname
    File::create(file_path)?
};

let content =response.text().await?;
//converts the String content into a byte slice (&[u8]). This is necessary because file writing operations typically work with bytes rather than strings.
copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}
