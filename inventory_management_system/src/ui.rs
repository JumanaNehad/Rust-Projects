use crate::inventory::Inventory;
use std::io::{self, Write};

pub fn start() {
      let mut inventory = Inventory::new();


    loop {
        println!("\nInventory Management System");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. View Inventory");
        println!("5. Record Sale");
        println!("6. View Sales");
        println!("7. Record Purchase");
        println!("8. View Purchases");
        println!("9. Generate Inventory Report");
        println!("10. Generate Sales Report");
        println!("11. Generate Purchase Report");
        println!("12. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let choice = loop {
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read input");
            match choice.trim().parse::<u32>() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Invalid input. Please enter a number between 1 and 12.");
                    print!("Enter your choice: ");
                    io::stdout().flush().unwrap();
                }
            }
        };

        match choice {
            1 => {
                let (name, description, price, quantity) = input_product_details();
                match inventory.add_product(name, description, price, quantity) {
                    Ok(()) => println!("Product Added successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }

            2 => {
                let (index, name, description, price, quantity) = input_edit_product_details();
                match inventory.edit_product(index, name, description, price, quantity) {
                    Ok(()) => println!("Product Edited successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }

            3 => {
                inventory.view_inventory();
                print!("Enter the product index to delete: ");
                io::stdout().flush().unwrap();
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read input");
                let index = index.trim().parse::<usize>().expect("Invalid input");
                match inventory.delete_product(index) {
                    Ok(()) => println!("Product deleted successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            4 => inventory.view_inventory(),
            5 => {
                let (product_name, quantity_sold, sale_price) = input_sale_details();
                match inventory.record_sale(&product_name, quantity_sold, sale_price) {
                    Ok(()) => println!("Sale recorded successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            6 => inventory.view_sales(),
            7 => {
                let (product_purchased, quantity_purchased, purchase_price) =
                    input_purchase_details();
                match inventory.record_purchase(
                    &product_purchased,
                    quantity_purchased,
                    purchase_price,
                ) {
                    Ok(()) => println!("Purchase recorded successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            8 => inventory.view_purchase(),
            9 => inventory.generate_inventory_report(),
            10 => inventory.generate_sales_report(),
            11 => inventory.generate_purchase_report(),
            12 => {
                println!("Exiting...");
                break;
            }

            _ => println!("Invalid choice, please try again."),
        }
    }

    fn input_product_details() -> (String, String, f64, i32) {
        println!("enter product name: ");
        io::stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");

        println!("enter product description: ");
        io::stdout().flush().unwrap();
        let mut description = String::new();
        io::stdin()
            .read_line(&mut description)
            .expect("Failed to read input");

        //price
        let price = loop {
            print!("Enter product price: ");
            io::stdout().flush().unwrap();
            let mut price = String::new();
            io::stdin()
                .read_line(&mut price)
                .expect("Failed to read input");
            match price.trim().parse::<f64>() {
                Ok(price) => break price,
                Err(_) => println!("Invalid input. Please enter a valid number for the price."),
            }
        };

        let quantity = loop {
            print!("Enter product quantity: ");
            io::stdout().flush().unwrap();
            let mut quantity = String::new();
            io::stdin()
                .read_line(&mut quantity)
                .expect("Failed to read input");
            match quantity.trim().parse::<i32>() {
                Ok(quantity) => break quantity,
                Err(_) => println!("Invalid input. Please enter a valid number for the quantity."),
            };
        };
        (
            name.trim().to_string(),
            description.trim().to_string(),
            price,
            quantity,
        )
    }

    fn input_edit_product_details() -> (usize, String, String, f64, i32) {
        println!("Enter product index ");
        io::stdout().flush().unwrap();
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read input");
        let index = index.trim().parse::<usize>().expect("Invalid input");

        let (name, description, price, quantity) = input_product_details();
        (index, name, description, price, quantity)
    }

    fn input_sale_details() -> (String, i32, f64) {
        print!("Enter product name: ");
        io::stdout().flush().unwrap();
        let mut product_name = String::new();
        io::stdin()
            .read_line(&mut product_name)
            .expect("Failed to read input");
    
        let quantity_sold = loop {
            print!("Enter quantity sold: ");
            io::stdout().flush().unwrap();
            let mut quantity_sold_str = String::new();
            io::stdin()
                .read_line(&mut quantity_sold_str)
                .expect("Failed to read input");
            match quantity_sold_str.trim().parse::<i32>() {
                Ok(quantity_sold) => break quantity_sold,
                Err(_) => println!("Invalid input. Please enter a valid number for the quantity sold."),
            }
        };
    
        let sale_price = loop {
            print!("Enter sale price: ");
            io::stdout().flush().unwrap();
            let mut sale_price_str = String::new();
            io::stdin()
                .read_line(&mut sale_price_str)
                .expect("Failed to read input");
            match sale_price_str.trim().parse::<f64>() {
                Ok(sale_price) => break sale_price,
                Err(_) => println!("Invalid input. Please enter a valid number for the sale price."),
            }
        };
    
        (product_name.trim().to_string(), quantity_sold, sale_price)
    }

   fn input_purchase_details() -> (String, i32, f64) {
    print!("Enter product name: ");
    io::stdout().flush().unwrap();
    let mut product_name = String::new();
    io::stdin()
        .read_line(&mut product_name)
        .expect("Failed to read input");

    let quantity_purchased = loop {
        print!("Enter quantity purchased: ");
        io::stdout().flush().unwrap();
        let mut quantity_purchased_str = String::new();
        io::stdin()
            .read_line(&mut quantity_purchased_str)
            .expect("Failed to read input");
        match quantity_purchased_str.trim().parse::<i32>() {
            Ok(quantity_purchased) => break quantity_purchased,
            Err(_) => println!("Invalid input. Please enter a valid number for the quantity purchased."),
        }
    };

    let purchase_price = loop {
        print!("Enter purchase price: ");
        io::stdout().flush().unwrap();
        let mut purchase_price_str = String::new();
        io::stdin()
            .read_line(&mut purchase_price_str)
            .expect("Failed to read input");
        match purchase_price_str.trim().parse::<f64>() {
            Ok(purchase_price) => break purchase_price,
            Err(_) => println!("Invalid input. Please enter a valid number for the purchase price."),
        }
    };

    (
        product_name.trim().to_string(),
        quantity_purchased,
        purchase_price,
    )
}
}
