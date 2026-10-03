use std::io;

fn main() {
    // Display the menu
    println!("======== RESTAURANT MENU ========");
    println!("P - Poundo Yam / Edinkaiko Soup   N3,200");
    println!("F - Fried Rice & Chicken          N3,000");
    println!("A - Amala & Ewedu Soup            N2,500");
    println!("E - Eba & Egusi Soup              N2,000");
    println!("W - White Rice & Stew             N2,500");
    println!("=================================");

    let mut total: f64 = 0.0;

    // Keep taking orders until the customer enters Q
    loop {
        println!("\nEnter food type (P, F, A, E or W), or Q to finish:");
        let mut food_input = String::new();
        io::stdin()
            .read_line(&mut food_input)
            .expect("Failed to read food type");
        let food_type = food_input.trim().to_uppercase();

        // Customer terminates the buying process
        if food_type == "Q" {
            break;
        }

        // Decide the price based on the letter
        let price: f64 = match food_type.as_str() {
            "P" => 3200.0,
            "F" => 3000.0,
            "A" => 2500.0,
            "E" => 2000.0,
            "W" => 2500.0,
            _ => {
                println!("Invalid food type. Please choose P, F, A, E, W or Q.");
                continue;
            }
        };

        // Read the quantity
        println!("Enter quantity:");
        let mut qty_input = String::new();
        io::stdin()
            .read_line(&mut qty_input)
            .expect("Failed to read quantity");
        let quantity: u32 = match qty_input.trim().parse() {
            Ok(q) if q > 0 => q,
            _ => {
                println!("Invalid quantity. Please enter a whole number greater than 0.");
                continue;
            }
        };

        // Add this order to the running total
        let cost = price * quantity as f64;
        total += cost;
        println!("Added N{:.2}. Running total: N{:.2}", cost, total);
    }

    // Final bill
    println!("\n========== FINAL BILL ==========");
    println!("Total charge: N{:.2}", total);

    if total > 10000.0 {
        let discount = total * 0.05;
        let final_total = total - discount;
        println!("Discount (5%): N{:.2}", discount);
        println!("Amount to pay: N{:.2}", final_total);
    } else {
        println!("No discount applied.");
        println!("Amount to pay: N{:.2}", total);
    }
    println!("Thank you for dining with us!");
}