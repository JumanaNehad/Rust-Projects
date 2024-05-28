#![deny(clippy::all)]

mod db;
use db::*;

//clear everything function
//clears the terminal screen where the program is running.
fn clr() {
    //27 as char converts the integer 27 to its corresponding ASCII character, which is the escape character (ESC).
    print!("{}[2J", 27 as char);
}

fn main() {
    //Making connection to the database
    let conn = init_database().expect("failed to initialize the database");

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

                //write to a database
                //we dont need file no more when created database
                write_password_to_db(
                    &conn, //created fo2
                    &entry.service,
                    &entry.username,
                    &entry.password,
                )
                .expect("Failed to write to DB");
                println!("Entry added successfully");
            }
            "2" => {
                clr();
                //services going to get from my json file
                //If there is an error print error and return an empty vector

                //reading from db instead of file
                let services = read_password_from_db(&conn).unwrap_or_else(|err| {
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

                let search = prompt("Search by service name:");
                //& anything means it's str not sting
                match search_service_by_name(&conn, &search) {
                    Ok(Some(entry)) => {
                        println!(
                            "
                        Services: {}
                        - Username: {}
                        - Password : {}",
                            entry.service, entry.username, entry.password
                        );
                    }
                    Ok(None) => {
                        println!("Service not found");
                    }
                    Err(err) => {
                        eprintln!("Error searching for service: {}", err);
                    }
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
