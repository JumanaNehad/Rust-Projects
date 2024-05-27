use serde::{Deserialize, Serialize};
use std::fs::File; //Provides functionalities to create and manipulate files.
use std::fs::OpenOptions; //Provides more flexible configurations for opening files.
use std::io;
use std::io::BufRead;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceInfo {
    pub service: String,
    pub username: String,
    pub password: String,
}

impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> Self {
        ServiceInfo {
            service,
            username,
            password,
        }
    }

    //read from json
    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_string)
    }

    #[allow(dead_code)]
    pub fn from_user_input() -> Self {
        println!("Enter password entry: ");
        let mut service = String::new();
        std::io::stdin()
            .read_line(&mut service)
            .expect("Failed to read line");

        println!("Enter username:  ");
        let mut username = String::new();
        std::io::stdin()
            .read_line(&mut username)
            .expect("Failed to read line");

        println!("Enter password : ");
        let mut password = String::new();
        std::io::stdin()
            .read_line(&mut password)
            .expect("Failed to read line");

        ServiceInfo::new(
            service.trim().to_string(),
            username.trim().to_string(),
            password.trim().to_string(),
        )
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("failed to serialize to JSON")
    }

    pub fn write_to_file(&self){
        let json_output = format!("{}\n",self.to_json());

        match OpenOptions::new()
        .create(true) //if file not created then create it
        .append(true) //if file exists then append 
        .open("passwords.json")//name of the file
        {
            //open the file
            Ok(mut file) => {
                //write to the file
                if let Err(e) = file.write_all(json_output.as_bytes()){
                    eprintln!("error waiting to file : {}",e)
                } else{
                    println!("Successfully wrote to file")
                }
            },
            Err(e)=> eprintln!("error opening file: {}",e)
         
        };
    }

    
} 
pub fn read_password_from_file() -> Result<Vec<ServiceInfo>,io::Error>{
    let file = File::open("passwords.json")?;
    let reader = io::BufReader::new(file);
    let mut services = Vec::new();

    //loop all file
    for line in reader.lines(){
        //for each line
        if let Ok(json_string) =line{
            //will get service info
            if let Ok(service_info)=ServiceInfo::from_json(&json_string){
                //one by one push it into vector
                services.push(service_info);
            }
        }
    }
    Ok(services)
}

pub fn prompt(prompt: &str) -> String {
    println!("{}",prompt);
    //This line ensures that anything written to stdout is immediately displayed to the user.
    io::stdout().flush().unwrap();
    let mut input = String::new();//initializes a mutable String to store the user's input.
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

