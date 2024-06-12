#![deny(clippy::all)]
mod auth;
mod inventory;
mod ui;

fn main() {
    println!("\nWelcome to Rusty Store Inventory Management System");

    // Authenticate user
    // if !auth::authentication() {
    //     println!("Authentication failed!");
    //     return;
    // }

    ui::start()
}
