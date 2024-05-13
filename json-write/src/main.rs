#![deny(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragragh {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    artical: String,
    author: String,
    //collection of struct
    paragragh: Vec<Paragragh>,
}

fn main() {
    let artical: Article = Article {
        artical: "json in rust".to_string(),
        author: "jumana nehad".to_string(),
        paragragh: vec![
            Paragragh {
                name: "joudy".to_string(),
            },
            Paragragh {
                name: "aly".to_string(),
            },
            Paragragh {
                name: "nehad".to_string(),
            },
        ],
    };

    //writing to json file
    let json = serde_json::to_string(&artical);
    match json {
        Ok(artical) => println!("the json is: {}", artical),
        Err(_) => println!("error"),
    }
    //   println!("the json is: {}", json);
}
