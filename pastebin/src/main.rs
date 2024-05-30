#![deny(clippy::all)]
//to run: cargo run and open http://localhost:8080/
//and in paste fun we can send this to anyone as an like in internet written in it anything
//Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust.
use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer, Responder}; //Responder: A trait that defines how to convert different types into an HTTP response.
use rand::distributions::Alphanumeric; //A distribution that generates random alphanumeric characters.
use rand::{thread_rng, Rng}; //use random generator to create the token , for every single user will have unique token //thread_rng: Returns a thread-local random number generator.//Rng: A trait that provides methods for generating random numbers.
use rusqlite::{params, Connection}; //to connect conections and pass the params to
use std::sync::Mutex; //Mutex to work with DB //This allows multiple threads to safely access the resource without data races. //NamedFile from Actix Files is used to serve static files, such as HTML, CSS, or JavaScript files.

//struct is used to share state (such as a database connection) across different parts of the application.
//Create struct to be able to connect to database
struct AppState {
    db: Mutex<Connection>, //It ensures that only one thread can access the data at a time, preventing data races.
}

// Struct for form data
#[derive(serde::Deserialize)]
struct FormData {
    content: String,
}

//This function is an asynchronous handler for the root route ("/") of your Actix web application.
async fn index() -> impl Responder{ //which is a trait that Actix uses to convert the return value into an HTTP response.
    HttpResponse::Ok().body(include_str!("index.html")) 
    //The body method sets the body of the HTTP response.include_str!("index.html") is a macro that includes the contents of the file index.html as a string. 
}

async fn submit(content: web::Form<FormData>, data: web::Data<AppState>) -> impl Responder {
    let token: String = thread_rng() //create token
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

        //open database connection
    let conn = data.db.lock().unwrap();
    conn.execute(
        "INSERT INTO pastes (token, content) VALUES (?, ?)",
        params![&token, &content.content],
    )
    .expect("Failed to insert into database");

    HttpResponse::SeeOther()
        .header("Location", format!("/paste/{}", token))
        .finish()
}

async fn get_paste(token: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let conn = data.db.lock().unwrap();
    let content = conn
        .query_row(
            "SELECT content FROM pastes WHERE token = ?",
            params![token.to_string()],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_else(|_| "Paste not found".to_string());

    HttpResponse::Ok().body(format!("<pre>{}</pre>", content))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
//create file db for me , open connection to db called "" if no connection 
let db = Connection::open("pates.db").expect("faild to open database");

db.execute(
    "CREATE TABLE IF NOT EXISTS pastes (token Text PRIMARY KEY , content TEXT)",
    params![], //When you have an SQL statement with placeholders, you can use params![] to provide the actual values for those placeholders.
).expect("Failed to create table");

//web::Data is an actix_web type that provides thread-safe shared state across the application.
let app_state =web::Data::new(AppState{
    db:Mutex::new(db),
});

 // Start the HTTP server
 HttpServer::new(move || {
    App::new() //This creates a new Actix web application.
        .app_data(app_state.clone()) //This adds shared application state to the app. The app_state is cloned and made available to all routes and services in the app.
        .service(web::resource("/style.css").to(|| {
            async { NamedFile::open("src/style.css") } //This sets up a service to serve the style.css file from the src directory. The file is served asynchronously using the NamedFile::open function.
        }))
        .route("/", web::get().to(index)) //root file shows the index file // It handles GET requests and routes them to the index function.
        .route("/submit", web::post().to(submit)) //It handles POST requests and routes them to the submit function.
        .route("/paste/{token}", web::get().to(get_paste)) //It handles GET requests and routes them to the get_paste
})
.bind("127.0.0.1:8080")? //This binds the server to the IP address 127.0.0.1 on port 8080
.run() //This runs the server and awaits its completion.
.await
}
