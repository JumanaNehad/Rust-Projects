#[derive(Debug)]
pub struct Product {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: i32,
}

pub struct Inventory {
    pub products: Vec<Product>,  //products vec
    pub sales: Vec<Sale>,        // vec lel ba3oh
    pub purchase: Vec<Purchase>, //vec lel eshtaroh
}

#[derive(Debug)]
pub struct Sale {
    pub product_name: String, //sold product
    pub quantity_sold: i32,
    pub sale_price: f64,
}

pub struct Purchase {
    pub product_purchased: String,
    pub quantity_purchased: i32,
    pub purchase_price: f64,
}

impl Inventory {
    // Initialize a new Inventory
    pub fn new() -> Self {
        Inventory {
            products: Vec::new(),
            sales: Vec::new(),
            purchase: Vec::new(),
        }
    }

    //will remove return type Product as we want to add this after adding product to the products vector
    pub fn add_product(
        &mut self,
        name: String,
        description: String,
        price: f64,
        quantity: i32,
    ) -> Result<(), String> {
        if name.is_empty() || description.is_empty() || price < 0.0 || quantity < 0 {
            return Err("Invalid product details".to_string());
        }
        let product = Product {
            name,
            description,
            price,
            quantity,
        };

        self.products.push(product);
        Ok(())
    }

    pub fn edit_product(
        &mut self,
        index: usize,
        name: String,
        description: String,
        price: f64,
        quantity: i32,
    ) -> Result<(), String> {
        //         This method attempts to get a mutable reference to the product at the specified index in the products vector.
        // If the index is within the bounds of the vector, it returns Some(&mut product), where product is a mutable reference to the product at the given index.
        // If the index is out of bounds, it returns None.
        if let Some(_product) = self.products.get_mut(index) {
            if name.is_empty() || description.is_empty() || price < 0.0 || quantity < 0 {
                return Err("Invalid product details".to_string());
            }
            //or make id for each product and compare
            Product {
                name,
                description,
                price,
                quantity,
            };
            Ok(())
        } else {
            return Err("\nProduct not found!".to_string());
        }
    }

    pub fn delete_product(&mut self, index: usize) -> Result<(), String> {
        if index <= self.products.len() {
            self.products.remove(index);
            Ok(())
        } else {
            Err("\nProduct not found!".to_string())
        }
    }

    pub fn view_inventory(&self) {
        for (index, product) in self.products.iter().enumerate() {
            println!(
                "{}: {} - {}, ${}, Quantity: {}",
                index, product.name, product.description, product.price, product.quantity
            );
        }
    }

    // Record a sale
    pub fn record_sale(
        &mut self,
        product_name: &str,
        quantity_sold: i32,
        sale_price: f64,
    ) -> Result<(), String> {
        if quantity_sold <= 0 || sale_price < 0.0 {
            return Err("Invalid sale details".to_string());
        }

        if let Some(product) = self.products.iter_mut().find(|p| p.name == product_name) {
            if product.quantity >= quantity_sold {
                product.quantity -= quantity_sold;
                self.sales.push(Sale {
                    product_name: product_name.to_string(),
                    quantity_sold,
                    sale_price,
                });
                Ok(())
            } else {
                Err("Insufficient stock".to_string())
            }
        } else {
            Err("Product not found".to_string())
        }
    }

    // View sales
    pub fn view_sales(&self) {
        for sale in &self.sales {
            println!(
                "Product: {}, Quantity Sold: {}, Sale Price: {}",
                sale.product_name, sale.quantity_sold, sale.sale_price
            );
        }
    }

    // Calculate total sales
    pub fn total_sales(&self) -> f64 {
        self.sales
            .iter()
            .map(|sale| sale.sale_price * sale.quantity_sold as f64)
            .sum()
    }

    pub fn total_profit(&self) -> f64 {
        self.sales
            .iter()
            .map(|sale| {
                // Find the cost price of the product by matching the product name in the sales record with the products list.
                let product_cost = self
                    .products
                    .iter()
                    .find(|p| p.name == sale.product_name)
                    .unwrap()
                    .price;

                // Calculate the profit for each sale by subtracting the cost price from the sale price and then multiplying by the quantity sold.
                (sale.sale_price - product_cost) * sale.quantity_sold as f64
            })
            .sum() // Sum up the profits for all sales.
    }

    //record purchase
    pub fn record_purchase(
        &mut self,
        product_purchased: &str,
        quantity_purchased: i32,
        purchase_price: f64,
    ) -> Result<(), String> {
        if quantity_purchased <= 0 || purchase_price < 0.0 {
            return Err("Invalid purchase details".to_string());
        }

        //if product name exists in product vec , then add quantity of the product and update p vec
        if let Some(product) = self
            .products
            .iter_mut()
            .find(|p| p.name == product_purchased)
        {
            product.quantity += quantity_purchased;
        } else {
            //we didnt buy this product before
            //so we have to add it
            self.products.push(Product {
                name: product_purchased.to_string(),
                description: "".to_string(),
                price: purchase_price,
                quantity: quantity_purchased,
            });
        }
        //update purchase vec in all cases
        self.purchase.push(Purchase {
            product_purchased: product_purchased.to_string(),
            quantity_purchased,
            purchase_price,
        });
        Ok(())
    }

    pub fn view_purchase(&self) {
        for p in &self.purchase {
            println!(
                "Product: {}, Quantity Purchased: {}, Purchase Price: {}",
                p.product_purchased, p.quantity_purchased, p.purchase_price
            );
        }
    }

    // Calculate total purchase cost
    pub fn total_purchase(&self) -> f64 {
        self.purchase
            .iter()
            .map(|p| p.purchase_price * p.quantity_purchased as f64)
            .sum()
    }

    // Generate inventory report
    pub fn generate_inventory_report(&self) {
        println!("\nInventory Report:");
        println!("-----------------");
        for (index, product) in self.products.iter().enumerate() {
            println!(
                "{}: {} - {}, ${}, Quantity: {}",
                index, product.name, product.description, product.price, product.quantity
            );
        }
    }

    // Generate sales report
    pub fn generate_sales_report(&self) {
        println!("\nSales Report:");
        println!("--------------");
        for sale in &self.sales {
            println!(
                "Product: {}, Quantity Sold: {}, Sale Price: ${}",
                sale.product_name, sale.quantity_sold, sale.sale_price
            );
        }
        println!("Total Sales: ${}", self.total_sales());
        println!("Total Profit: ${}", self.total_profit());
    }

    // Generate purchase report
    pub fn generate_purchase_report(&self) {
        println!("\nPurchase Report:");
        println!("-----------------");
        for purchase in &self.purchase {
            println!(
                "Product: {}, Quantity Purchased: {}, Purchase Price: ${}",
                purchase.product_purchased, purchase.quantity_purchased, purchase.purchase_price
            );
        }
        println!("Total Purchase Cost: ${}", self.total_purchase());
    }
}
