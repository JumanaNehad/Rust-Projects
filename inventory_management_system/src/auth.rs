pub fn authentication()->bool{
println!("Enter your username:");
let mut username=String::new();
std::io::stdin().read_line(&mut username).expect("Failed to read line");
println!("Enter your Password:");
let mut password=String::new();
std::io::stdin().read_line(&mut password).expect("Failed to read line");

  // Placeholder authentication check
  username.trim() == "Jumana" && password.trim() == "password"
}