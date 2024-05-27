#![deny(clippy::all)]

mod pentry;
//prompt fun help me print out stuff and accept values from the user
use crate::pentry::{prompt, read_password_from_file};

use crate::pentry::ServiceInfo;

//clear everything function
//clears the terminal screen where the program is running.
fn clr() {
    //27 as char converts the integer 27 to its corresponding ASCII character, which is the escape character (ESC).
    print!("{}[2J", 27 as char);
}

fn main() {
    clr();
    //add ascii art from (asci art generator app)
    let ascii = r#"
    .__   __   
    ___________    ______ ______ ___  _______   __ __|  |_/  |_ 
    \____ \__  \  /  ___//  ___/ \  \/ /\__  \ |  |  \  |\   __\
    |  |_> > __ \_\___ \ \___ \   \   /  / __ \|  |  /  |_|  |  
    |   __(____  /____  >____  >   \_/  (____  /____/|____/__|  
    |__|       \/     \/     \/              \/                 
    "#;

    println!("{ascii}");
    //loop through all the options
    loop {
        println!("Password manger menu");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search Entry");
        println!("4. Quit");

        //we will takes the choice from the user
        //hasab the choice will call the relevant fun depending on through match
        let mut choice = String::new();
        //This line reads a line of input from the standard input
        //read_line method takes a mutable reference to a string where the input is stored.
        std::io::stdin().read_line(&mut choice).unwrap();

        //trim it as there could be spaces we dont want from both ends of the input string.
        match choice.trim() {
            "1" => {
                clr();
                //struct in another file
                let entry =
                    ServiceInfo::new(prompt("Service:"), prompt("Username:"), prompt("Password:"));
                println!("Entry added successfully");
                entry.write_to_file();
            }
            "2" => {
                clr();
                //services going to get from my json file
                //If there is an error print error and return an empty vector
                let services = read_password_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    //service will going in to a vector help us print all the services
                    Vec::new()
                });
                //loop over the services
                for item in &services {
                    println!(
                        "
                    Services: {}
                    - Username: {}
                    - Password : {}",
                        item.service, item.username, item.password
                    );
                }
            }
            "3" => {
                clr();
                //to search get all services then loop over them to match with the I am searching for
                let services = read_password_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    //and return an empty vector
                    Vec::new()
                });
                //displays this prompt to the user and waits for their input. The input (search term) is then stored in the variable search.
                let search = prompt("Search :");
                let mut found = false; // Flag to track if any services are found.
                for item in &services {
                    if item.service.as_str() == search.as_str() {
                        println!(
                            "
                        Services: {}
                        - Username: {}
                        - Password : {}",
                            item.service, item.username, item.password
                        );
                        found = true;
                    }
                }
                if !found {
                    println!("No services found matching '{}'.", search);
                }
            }
            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice"),
        }
        println!("\n\n");
    }
}
