#![deny(clippy::all)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    artical: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"{
        "artical": "The Rise of Rust",
        "author": "Jane Doe",
        "paragraph": [
            { "name": "Introduction" },
            { "name": "Body" },
            { "name": "Conclusion" }
        ]
    }"#;

    // let parsed = read_json_typed(json);
    // println!("{}",parsed.unwrap().paragraph[0].name);

    //Error handling
    match read_json_typed(json) {
        Ok(article) => {
            if let Some(paragraph) = article.paragraph.get(1) {
                println!("Paragragh name: {}", paragraph.name);
            } else {
                println!("No paragraphs found.");
            }
        }
        //if no paragragh vec
        Err(e) => println!("Failed to parse JSON: {}", e),
    }
}
//or return Article
fn read_json_typed(json: &str) -> Result<Article, serde_json::Error> {
    serde_json::from_str(json)
}
