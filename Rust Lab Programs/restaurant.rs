use std::io;

enum MenuItem {
    Burger,
    Pizza,
    Pasta,
}

fn main() {
    println!("Enter the menu item (Burger, Pizza, Pasta):");
    let mut item = String::new();
    io::stdin().read_line(&mut item).expect("Failed to read line");
    let item = item.trim();
    println!("Enter the quantity:");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read line");
    let quantity: i32 = quantity.trim().parse().expect("Please type a number!");
    let menu_item = match item {
        "Burger" => MenuItem::Burger,
        "Pizza" => MenuItem::Pizza,
        "Pasta" => MenuItem::Pasta,
        _ => panic!("Invalid menu item"),
    };
    let price = match menu_item {
        MenuItem::Burger => 40.0,
        MenuItem::Pizza => 100.0,
        MenuItem::Pasta => 120.0,
    };
    let discount = if quantity > 5 { 0.10 } else { 0.0 };
    let total_price = price * quantity as f64 * (1.0 - discount);
    println!("Total price: ₹{:.2}", total_price);
}